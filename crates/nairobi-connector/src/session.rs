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

//! NeuralSession — AT-SPI2 session management with RegistryLock and heartbeat.

use crate::action;
use crate::engine::DFSEngine;
use crate::error::{NeuralError, Result};
use crate::safety::WindowLock;
use crate::toon;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use tokio::sync::RwLock;
use zbus::Connection;

/// The AT-SPI2 registry well-known bus name.
const REGISTRY_DEST: &str = "org.a11y.atspi.Registry";
/// The root accessible object path in the AT-SPI2 registry.
const REGISTRY_ROOT_PATH: &str = "/org/a11y/atspi/accessible/root";

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

    /// Find a window by title substring using simple DFS over registry children.
    pub async fn find_window(&self, title_substring: &str) -> Result<(String, String)> {
        let proxy = DFSEngine::timeout_proxy_build(
            &self.connection,
            REGISTRY_DEST,
            REGISTRY_ROOT_PATH,
        )
        .await?;
        let apps = DFSEngine::get_children(&proxy).await;
        let lower_title = title_substring.to_lowercase();

        for app in &apps {
            let app_dest = app.name_as_str().unwrap_or_default();
            let app_path = app.path_as_str();
            let app_proxy = match DFSEngine::timeout_proxy_build(
                &self.connection,
                app_dest,
                app_path,
            )
            .await
            {
                Ok(p) => p,
                Err(_) => continue,
            };
            let name = DFSEngine::get_name(&app_proxy).await;
            if name.to_lowercase().contains(&lower_title) {
                let mut dest_lock = self.target_destination.write().await;
                let mut path_lock = self.target_path.write().await;
                *dest_lock = Some(app_dest.to_string());
                *path_lock = Some(app_path.to_string());
                self.touch_activity().await;
                return Ok((app_dest.to_string(), app_path.to_string()));
            }
            // Also check frame children
            let windows = DFSEngine::get_children(&app_proxy).await;
            for w in &windows {
                let w_dest = w.name_as_str().unwrap_or_default();
                let w_path = w.path_as_str();
                let w_proxy = match DFSEngine::timeout_proxy_build(
                    &self.connection,
                    w_dest,
                    w_path,
                )
                .await
                {
                    Ok(p) => p,
                    Err(_) => continue,
                };
                let w_name = DFSEngine::get_name(&w_proxy).await;
                if w_name.to_lowercase().contains(&lower_title) {
                    let mut dest_lock = self.target_destination.write().await;
                    let mut path_lock = self.target_path.write().await;
                    *dest_lock = Some(w_dest.to_string());
                    *path_lock = Some(w_path.to_string());
                    self.touch_activity().await;
                    return Ok((w_dest.to_string(), w_path.to_string()));
                }
            }
        }

        Err(NeuralError::NodeNotFound(format!(
            "Window containing '{}'",
            title_substring
        )))
    }

    /// Get the cached target window, performing a heartbeat check.
    pub async fn get_cached_window(&self) -> Result<(String, String)> {
        let dest_opt = self.target_destination.read().await.clone();
        let path_opt = self.target_path.read().await.clone();
        match (dest_opt, path_opt) {
            (Some(dest), Some(path)) => {
                let proxy = DFSEngine::timeout_proxy_build(&self.connection, &dest, &path).await?;
                let role = DFSEngine::get_role(&proxy).await;
                if role != atspi::Role::Unknown {
                    self.touch_activity().await;
                    Ok((dest, path))
                } else {
                    let mut d = self.target_destination.write().await;
                    let mut p = self.target_path.write().await;
                    *d = None;
                    *p = None;
                    Err(NeuralError::NodeNotFound(
                        "Cached window is no longer alive".to_string(),
                    ))
                }
            }
            _ => Err(NeuralError::NodeNotFound("No cached window".to_string())),
        }
    }

    /// Generate a TOON string for the currently cached window.
    /// Also caches the ID → (destination, object_path) mapping for action dispatch.
    pub async fn get_ui_map(&self, max_depth: u32) -> Result<(String, u32, u128)> {
        let (dest, path) = self.get_cached_window().await?;
        let proxy =
            DFSEngine::timeout_proxy_build(&self.connection, &dest, &path)
                .await?;
        let snapshot = DFSEngine::capture_snapshot(proxy).await;
        self.touch_activity().await;
        let (toon_str, node_count, elapsed, new_id_map) =
            toon::generate_toon(&snapshot, max_depth);

        // Cache the ID map for subsequent interact_by_id / type_text_by_id calls
        let mut map = self.id_map.write().await;
        *map = new_id_map;
        tracing::debug!("[SESSION] Cached {} node ID mappings", map.len());

        Ok((toon_str, node_count, elapsed))
    }

    /// Lock a target application for safety (RegistryLock).
    pub async fn lock_window(&self, app_name: &str, window_title: &str) -> Result<()> {
        let lock = WindowLock::new(app_name.to_string(), window_title.to_string());
        if lock.is_locked() {
            return Err(NeuralError::WindowLocked(format!(
                "Window '{}' ({}) is sensitive and locked",
                window_title, app_name
            )));
        }
        let mut lock_guard = self.window_lock.write().await;
        *lock_guard = Some(lock);
        self.touch_activity().await;
        Ok(())
    }

    /// Release the current RegistryLock.
    pub async fn release_lock(&self) {
        let mut lock_guard = self.window_lock.write().await;
        *lock_guard = None;
        tracing::info!("[SAFETY] RegistryLock released");
    }

    /// Update the last activity timestamp (called by heartbeat-aware operations).
    pub async fn touch_activity(&self) {
        let mut last = self.last_activity.write().await;
        *last = Instant::now();
        self.heartbeat_fired.store(false, Ordering::SeqCst);
    }

    /// Check if heartbeat has timed out and release lock if so.
    pub async fn check_heartbeat(&self) {
        if self.heartbeat_fired.load(Ordering::SeqCst) {
            return;
        }
        let last = *self.last_activity.read().await;
        if last.elapsed() > crate::safety::HEARTBEAT_TIMEOUT {
            self.heartbeat_fired.store(true, Ordering::SeqCst);
            tracing::warn!(
                "[HEARTBEAT] No activity for {}ms — releasing RegistryLock",
                crate::safety::HEARTBEAT_TIMEOUT.as_millis()
            );
            self.release_lock().await;
        }
    }

    /// Execute a semantic action on a node identified by TOON ID.
    /// Requires that `get_ui_map()` was called first to populate the ID map.
    pub async fn interact_by_id(&self, node_id: u32, action_name: &str) -> Result<String> {
        // Resolve node ID to D-Bus coordinates
        let (dest, path) = {
            let map = self.id_map.read().await;
            map.get(&node_id)
                .cloned()
                .ok_or_else(|| NeuralError::NodeNotFound(format!(
                    "Node ID {} not found — call nairobi_get_ui_map first to refresh IDs",
                    node_id
                )))?
        };

        match action_name {
            "click" | "activate" => {
                action::do_action(
                    &self.connection,
                    &dest,
                    &path,
                    0,
                )
                .await?;
                self.touch_activity().await;
                Ok(format!("Action '{}' executed on node {} (path={})", action_name, node_id, path))
            }
            "focus" => {
                action::grab_focus(
                    &self.connection,
                    &dest,
                    &path,
                )
                .await?;
                self.touch_activity().await;
                Ok(format!("Focus set on node {} (path={})", node_id, path))
            }
            _ => Err(NeuralError::ActionFailed(format!("Unknown action: {}", action_name))),
        }
    }

    /// Type text into an editable field identified by TOON ID.
    /// Requires that `get_ui_map()` was called first to populate the ID map.
    pub async fn type_text_by_id(&self, node_id: u32, text: &str) -> Result<String> {
        // Resolve node ID to D-Bus coordinates
        let (dest, path) = {
            let map = self.id_map.read().await;
            map.get(&node_id)
                .cloned()
                .ok_or_else(|| NeuralError::NodeNotFound(format!(
                    "Node ID {} not found — call nairobi_get_ui_map first to refresh IDs",
                    node_id
                )))?
        };

        action::set_text(
            &self.connection,
            &dest,
            &path,
            text,
        )
        .await?;
        self.touch_activity().await;
        Ok(format!("Text set on node {} (path={}): {}", node_id, path, text))
    }
}