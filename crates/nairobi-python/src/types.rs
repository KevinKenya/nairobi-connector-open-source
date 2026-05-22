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

// File: /home/KevinKenya/nairobi-connector-open-source/crates/nairobi-python/src/types.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

// nairobi-open-source-release/crates/nairobi-python/src/types.rs
use std::collections::HashMap;
use std::os::unix::io::{AsRawFd, FromRawFd};
use tokio::runtime::Runtime;
use tokio::sync::RwLock;
use std::sync::OnceLock;
use zbus::zvariant::OwnedFd;
use nairobi_hub::RefineryClient;

// ── Persistent Infrastructure ──────────────────────────────────────
// A single Tokio runtime and D-Bus connection shared across all bridge calls.
// This eliminates ~300-400ms of overhead per call from Runtime::new() + Connection::session().

static GLOBAL_RUNTIME: once_cell::sync::OnceCell<Runtime> = once_cell::sync::OnceCell::new();
static GLOBAL_CLIENT: OnceLock<tokio::sync::OnceCell<RefineryClient>> = OnceLock::new();

/// Returns the shared Tokio runtime (created once on first use).
pub fn get_runtime() -> pyo3::PyResult<&'static Runtime> {
    GLOBAL_RUNTIME.get_or_try_init(|| {
        Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create persistent Tokio runtime: {}", e))
        })
    })
}

/// Returns the shared D-Bus RefineryClient (connected once on first use).
pub fn get_client() -> &'static tokio::sync::OnceCell<RefineryClient> {
    GLOBAL_CLIENT.get_or_init(|| tokio::sync::OnceCell::new())
}

/// Connects the persistent client if not already connected.
/// Must be called within the global runtime context.
pub async fn ensure_client() -> Result<&'static RefineryClient, nairobi_protocol::ImperialError> {
    let cell = get_client();
    cell.get_or_try_init(|| async {
        RefineryClient::connect().await
    }).await
}

// ── Handle Registry (unchanged) ────────────────────────────────────

pub static HANDLE_REGISTRY: OnceLock<HandleRegistry> = OnceLock::new();

pub fn get_registry() -> &'static HandleRegistry {
    HANDLE_REGISTRY.get_or_init(|| HandleRegistry::new())
}

/// Internal handle registry mapping UUID strings to OwnedFd.
/// This is the Python layer's view of the memfd registry.
pub struct HandleRegistry {
    pub map: RwLock<HashMap<String, OwnedFd>>,
}

impl HandleRegistry {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub async fn insert(&self, uuid: String, fd: OwnedFd) {
        let mut map = self.map.write().await;
        map.insert(uuid, fd);
    }

    pub async fn get(&self, uuid: &str) -> Option<OwnedFd> {
        let map = self.map.read().await;
        map.get(uuid).map(|fd| unsafe {
            let raw = fd.as_raw_fd();
            let duped = libc::dup(raw);
            OwnedFd::from_raw_fd(duped)
        })
    }

    pub async fn remove(&self, uuid: &str) -> Option<OwnedFd> {
        let mut map = self.map.write().await;
        map.remove(uuid)
    }
}

/// Map ImperialError to Python's RuntimeError
pub fn map_imperial_error(e: nairobi_protocol::ImperialError) -> pyo3::PyErr {
    pyo3::exceptions::PyRuntimeError::new_err(e.to_string())
}
