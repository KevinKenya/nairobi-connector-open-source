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
/// Find a window by title substring using DFS over registry children with exponential backoff and timeout.
    pub async fn find_window(&self, title_substring: &str) -> Result<(String, String)> {
        let lower_title = title_substring.to_lowercase();
        let start_time = Instant::now();
        let hard_timeout = Duration::from_millis(3500);

        tracing::info!(
            "[SESSION] Polling AT-SPI2 Registry for window containing '{}' (max 3.5s)...",
            title_substring
        );

        let search_task = async {
            for attempt in 0..5 {
                if attempt > 0 {
                    let backoff_ms = 150 * (2_u64.pow(attempt - 1));
                    let backoff_dur = Duration::from_millis(backoff_ms);
                    if start_time.elapsed() + backoff_dur >= hard_timeout {
                        break;
                    }
                    sleep(backoff_dur).await;
                }

                let proxy = match DFSEngine::timeout_proxy_build(
                    &self.connection,
                    REGISTRY_DEST,
                    REGISTRY_ROOT_PATH,
                )
                .await
                {
                    Ok(p) => p,
                    Err(_) => continue,
                };

                let apps: Vec<Accessible> = DFSEngine::get_children(&proxy).await;

                for app in apps {
                    let app_dest: String = app.name.clone();
                    let app_path: String = app.path.to_string();

                    // 1. Destination Matching (Case-insensitive)
                    let dest_match = app_dest.to_lowercase().contains(&lower_title);

                    let app_proxy = match DFSEngine::timeout_proxy_build(
                        &self.connection,
                        &app_dest,
                        &app_path,
                    )
                    .await
                    {
                        Ok(p) => p,
                        Err(_) => continue,
                    };

                    // 2. Name Matching (Case-insensitive)
                    let app_name = DFSEngine::get_name(&app_proxy).await;
                    let name_match = app_name.to_lowercase().contains(&lower_title);

                    if dest_match || name_match {
                        // Found a candidate application.

                        // 3. Handle Empty Trees: sleep 100ms and re-query once.
                        let mut children: Vec<Accessible> = DFSEngine::get_children(&app_proxy).await;
                        if children.is_empty() {
                            sleep(Duration::from_millis(100)).await;
                            children = DFSEngine::get_children(&app_proxy).await;
                        }

                        // If still no children after retry, do not return yet as it's not fully rendered.
                        if children.is_empty() {
                            tracing::debug!("[SESSION] Candidate app '{}' found but tree is empty after 100ms.", app_dest);
                        } else {
                            // Search depth: App -> Child (L2) -> Grandchild (L3)
                            if name_match {
                                return Ok((app_dest, app_path));
                            }

                            // Search children (Level 2)
                            for child in children {
                                let c_dest: String = child.name.clone();
                                let c_path: String = child.path.to_string();
                                let c_proxy = match DFSEngine::timeout_proxy_build(
                                    &self.connection,
                                    &c_dest,
                                    &c_path,
                                )
                                .await
                                {
                                    Ok(p) => p,
                                    Err(_) => continue,
                                };

                                let c_name = DFSEngine::get_name(&c_proxy).await;
                                if c_name.to_lowercase().contains(&lower_title) {
                                    return Ok((c_dest, c_path));
                                }

                                // Search grandchildren (Level 3)
                                let grandchildren = DFSEngine::get_children(&c_proxy).await;
                                for gc in grandchildren {
                                    let gc_dest: String = gc.name.clone();
                                    let gc_path: String = gc.path.to_string();
                                    let gc_proxy = match DFSEngine::timeout_proxy_build(
                                        &self.connection,
                                        &gc_dest,
                                        &gc_path,
                                    )
                                    .await
                                    {
                                        Ok(p) => p,
                                        Err(_) => continue,
                                    };

                                    let gc_name = DFSEngine::get_name(&gc_proxy).await;
                                    if gc_name.to_lowercase().contains(&lower_title) {
                                        return Ok((gc_dest, gc_path));
                                    }
                                }
                            }

                            // Fallback to the application node if destination matched but no specific window title matched.
                            if dest_match {
                                return Ok((app_dest, app_path));
                            }
                        }
                    }
                }
            }

            Err(NeuralError::NodeNotFound(format!(
                "Window containing '{}' not registered on the AT-SPI2 bus after 5 attempts",
                title_substring
            )))
        };

        match tokio::time::timeout(hard_timeout, search_task).await {
            Ok(inner_result) => {
                if let Ok((ref dest, ref path)) = inner_result {
                    let mut dest_lock = self.target_destination.write().await;
                    let mut path_lock = self.target_path.write().await;
                    *dest_lock = Some(dest.clone());
                    *path_lock = Some(path.clone());
                    self.touch_activity().await;
                    tracing::info!(
                        "[SESSION] Found matching window: '{}' after {}ms",
                        title_substring,
                        start_time.elapsed().as_millis()
                    );
                }
                inner_result
            }
            Err(_) => Err(NeuralError::Timeout(
                "Window search timed out after 3.5s".to_string(),
            )),
        }
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
    /// Also caches the ID → (destination, object path) mapping for action dispatch.
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

    /// Wait for a human to save an edit staged via `nairobi_type_text`.
    ///
    /// This does not save anything itself — it is a read-only observer. Starting
    /// from the given node, it walks up the accessibility tree to the nearest
    /// `Frame` or `PageTab` ancestor (the level at which most editors surface an
    /// "unsaved changes" indicator) and polls that ancestor's name for the
    /// disappearance of a dirty-buffer marker (`•` or `*`), which is how GTK,
    /// GNOME Text Editor, and most GtkSourceView-based apps signal an unsaved
    /// document. Returns once the marker clears, or times out.
    ///
    /// This lets an agent stage an edit and then wait for explicit human
    /// confirmation (the human pressing Ctrl+S) before treating the edit as
    /// committed, rather than assuming a `set_text` call succeeded.
    pub async fn wait_for_save(&self, node_id: u32, timeout_secs: u64) -> Result<(bool, String, String)> {
        let (dest, path) = {
            let map = self.id_map.read().await;
            map.get(&node_id)
                .cloned()
                .ok_or_else(|| NeuralError::NodeNotFound(format!(
                    "Node ID {} not found — call nairobi_get_ui_map first to refresh IDs",
                    node_id
                )))?
        };

        // Walk up to the nearest Frame or PageTab ancestor, capped at 32 hops
        // to guard against a malformed or cyclic accessibility tree.
        let mut current_dest = dest;
        let mut current_path = path;
        let mut ancestor: Option<(String, String)> = None;

        for _ in 0..32 {
            let proxy = DFSEngine::timeout_proxy_build(&self.connection, &current_dest, &current_path).await?;
            let role = DFSEngine::get_role(&proxy).await;

            if is_save_boundary_role(role) {
                ancestor = Some((current_dest.clone(), current_path.clone()));
                break;
            }

            match DFSEngine::get_parent(&proxy).await {
                Some((pd, pp)) => {
                    current_dest = pd;
                    current_path = pp;
                }
                None => break,
            }
        }

        let (a_dest, a_path) = ancestor.ok_or_else(|| {
            NeuralError::NodeNotFound(
                "Could not resolve a Frame or PageTab ancestor for node — is it inside an editor window?".to_string(),
            )
        })?;

        let ancestor_proxy = DFSEngine::timeout_proxy_build(&self.connection, &a_dest, &a_path).await?;
        let initial_name = DFSEngine::get_name(&ancestor_proxy).await;

        tracing::info!(
            "[SESSION] wait_for_save watching ancestor: {} (name: '{}')",
            a_path, initial_name
        );

        let start = Instant::now();
        let timeout_dur = Duration::from_secs(timeout_secs);

        while start.elapsed() < timeout_dur {
            let proxy = DFSEngine::timeout_proxy_build(&self.connection, &a_dest, &a_path).await?;
            let current_name = DFSEngine::get_name(&proxy).await;

            if !has_dirty_marker(&current_name) {
                self.touch_activity().await;
                return Ok((true, initial_name, current_name));
            }

            sleep(Duration::from_millis(500)).await;
        }

        let final_name = DFSEngine::get_name(&ancestor_proxy).await;
        Ok((false, initial_name, final_name))
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

// ─── Unit tests ──────────────────────────────────────────────────────────────
//
// These cover the pure, D-Bus-free logic in this module. They run in any
// environment (CI included) with no AT-SPI2 registry or session bus required.
//
// Live-desktop behavior (does `wait_for_save` actually observe a real GNOME
// Text Editor save?) is NOT something a unit test can honestly claim to prove —
// that needs a real accessibility tree. See
// `tests/verify_save_manual.rs` for the `#[ignore]`d manual harness for that.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dirty_marker_bullet_detected() {
        assert!(has_dirty_marker("\u{2022} untitled.txt"));
    }

    #[test]
    fn dirty_marker_asterisk_detected() {
        assert!(has_dirty_marker("*untitled.txt"));
    }

    #[test]
    fn dirty_marker_asterisk_mid_title_detected() {
        // Some editors put the marker after the name, not before it —
        // this must not assume a fixed position.
        assert!(has_dirty_marker("untitled.txt *"));
    }

    #[test]
    fn clean_title_has_no_marker() {
        assert!(!has_dirty_marker("untitled.txt"));
    }

    #[test]
    fn empty_title_has_no_marker() {
        assert!(!has_dirty_marker(""));
    }

    #[test]
    fn asterisk_in_title_text_is_a_documented_false_positive() {
        // A title like "3 * 4 = 12.txt" WILL be flagged as dirty even when it
        // isn't. This is a known limitation, not a bug: substring matching on
        // '*' can't distinguish a dirty-marker from a literal asterisk without
        // also cross-checking the AT-SPI STATE_MODIFIED bit, which is a real
        // follow-up (see module doc). Asserted here so the limitation stays a
        // visible, tested fact instead of a silent gap discovered in the field.
        assert!(has_dirty_marker("3 * 4 = 12.txt"));
    }

    #[test]
    fn frame_is_a_save_boundary() {
        assert!(is_save_boundary_role(atspi::Role::Frame));
    }

    #[test]
    fn page_tab_is_a_save_boundary() {
        assert!(is_save_boundary_role(atspi::Role::PageTab));
    }

    #[test]
    fn panel_is_not_a_save_boundary() {
        assert!(!is_save_boundary_role(atspi::Role::Panel));
    }

    #[test]
    fn push_button_is_not_a_save_boundary() {
        assert!(!is_save_boundary_role(atspi::Role::PushButton));
    }

    #[tokio::test]
    async fn wait_for_save_rejects_unknown_node_id_before_touching_dbus() {
        // establish() opens a real session-bus connection, which is available
        // in effectively every Linux CI runner (a session bus doesn't require
        // AT-SPI2 or a display) — unlike find_window/get_ui_map, this path
        // never talks to the AT-SPI2 registry, so it's safe to run unconditionally.
        let session = match NeuralSession::establish().await {
            Ok(s) => s,
            Err(_) => {
                eprintln!("skipping: no D-Bus session bus available in this environment");
                return;
            }
        };

        let result = session.wait_for_save(999, 1).await;
        assert!(matches!(result, Err(NeuralError::NodeNotFound(_))));
    }
}



