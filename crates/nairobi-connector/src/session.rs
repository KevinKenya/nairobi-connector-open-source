// Copyright 2026 Kevin Chege
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// crates/nairobi-connector/src/session.rs
// Author: Kevin Chege, Location: Nairobi, Date: 21st May 2026

//! NeuralSession — AT-SPI2 session management with RegistryLock and heartbeat.

use crate::action;
use crate::engine::DFSEngine;
use crate::error::{NeuralError, Result};
use crate::safety::WindowLock;
use crate::toon;
use atspi::Accessible;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::sleep;
use zbus::Connection;

/// The AT-SPI2 registry well-known bus name.
const REGISTRY_DEST: &str = "org.a11y.atspi.Registry";
/// The root accessible object path in the AT-SPI2 registry.
const REGISTRY_ROOT_PATH: &str = "/org/a11y/atspi/accessible/root";

/// Whether a window/tab title contains an unsaved-changes marker.
/// GTK, GNOME Text Editor, and most GtkSourceView-based apps prepend `•` or
/// `*` to the title while there are unsaved changes, and drop it on save.
/// Pure and D-Bus-free so it can be unit tested without a live AT-SPI2 session.
pub fn has_dirty_marker(title: &str) -> bool {
    title.contains('\u{2022}') || title.contains('*')
}

/// Whether a role marks the boundary `wait_for_save` should stop climbing at.
/// `Frame` covers standalone windows; `PageTab` covers tabbed editors (e.g. a
/// tab in GNOME Text Editor) where the dirty marker lives on the tab, not the
/// outer window. Pure and D-Bus-free so it can be unit tested directly.
pub fn is_save_boundary_role(role: atspi::Role) -> bool {
    matches!(role, atspi::Role::Frame | atspi::Role::PageTab)
}

/// Manages an AT-SPI2 session with target window tracking and RegistryLock safety.
pub struct NeuralSession {
    connection: Connection,
    /// Target application destination (D-Bus name).
    pub target_destination: RwLock<Option<String>>,
    /// Target window object path.
    pub target_path: RwLock<Option<String>>,
    /// RegistryLock for safety.
    pub window_lock: RwLock<Option<WindowLock>>,
    /// Last activity timestamp for heartbeat monitoring.
    pub last_activity: RwLock<Instant>,
    /// Whether a heartbeat timeout has fired.
    pub heartbeat_fired: AtomicBool,
    /// Maps sequential TOON node IDs to (D-Bus destination, object path) pairs.
    /// Populated by `get_ui_map()` and consumed by `interact_by_id()` / `type_text_by_id()`.
    pub id_map: RwLock<HashMap<u32, (String, String)>>,
}

impl NeuralSession {
    /// Establish a connection to the AT-SPI2 session bus.
    pub async fn establish() -> Result<Self> {
        let connection = Connection::session()
            .await
            .map_err(|e| NeuralError::ZbusError(format!("Failed to connect to session bus: {}", e)))?;
        Ok(Self {
            connection,
            target_destination: RwLock::new(None),
            target_path: RwLock::new(None),
            window_lock: RwLock::new(None),
            last_activity: RwLock::new(Instant::now()),
            heartbeat_fired: AtomicBool::new(false),
            id_map: RwLock::new(HashMap::new()),
        })
    }

    /// Get the AT-SPI2 connection reference.
    pub fn connection(&self) -> &Connection {
        &self.connection
    }

