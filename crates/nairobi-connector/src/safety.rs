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

// crates/nairobi-connector/src/safety.rs
// Author: Kevin Chege, Location: Nairobi, Date: 20th May 2026

//! Registry Lock — Safety guard to prevent AI from interacting with sensitive system windows.
//!
//! The RegistryLock ensures that the AI cannot click, type, or focus on:
//! - Password prompts (gnome-keyring, polkit, sudo dialogs)
//! - System settings panels (gnome-shell, gnome-system-monitor)
//! - Authentication flows (seahorse, authenticator)
//!
//! If a heartbeat timeout occurs, the lock is automatically released to prevent
//! the OS from being left in a "paralyzed" state.

use std::time::{Duration, Instant};

/// List of sensitive applications that should be locked from AI interaction.
pub const SENSITIVE_APPS: &[&str] = &[
    "gnome-shell",
    "gnome-keyring",
    "gdm",
    "polkit-gnome-authentication-agent-1",
    "gnome-system-monitor",
    "seahorse",
    "authenticator",
    "gnome-characters",
    "gnome-initial-setup",
];

/// Keywords in window titles that indicate sensitive content.
pub const SENSITIVE_KEYWORDS: &[&str] = &[
    "password",
    "pin",
    "authenticate",
    "authentication",
    "sudo",
    "unlock",
    "login",
    "credentials",
    "secret",
    "keyring",
    "passphrase",
];

/// Maximum time a lock can be held without heartbeat activity before auto-release.
pub const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);

/// Represents a locked window target for safety.
#[derive(Debug, Clone)]
pub struct WindowLock {
    /// Application name (D-Bus sender name).
    pub app_name: String,
    /// Window title detected during lock.
    pub window_title: String,
    /// Whether this window is shielded (sensitive).
    pub is_shielded: bool,
    /// When the lock was acquired.
    pub locked_at: Instant,
}

impl WindowLock {
    /// Create a new WindowLock.
    pub fn new(app_name: String, window_title: String) -> Self {
        let is_shielded = Self::check_window(&app_name, &window_title);
        Self {
            app_name,
            window_title,
            is_shielded,
            locked_at: Instant::now(),
        }
    }

    /// Check if an application + window title matches sensitive patterns.
    pub fn check_window(app_name: &str, window_title: &str) -> bool {
        let app_lower = app_name.to_lowercase();
        let title_lower = window_title.to_lowercase();

        // Check if app is in sensitive apps list
        if SENSITIVE_APPS.iter().any(|&a| app_lower.contains(a)) {
            return true;
        }

        // Check if window title contains sensitive keywords
        if SENSITIVE_KEYWORDS.iter().any(|&k| title_lower.contains(k)) {
            return true;
        }

        false
    }

    /// Whether this lock is currently active and shielding a sensitive window.
    pub fn is_locked(&self) -> bool {
        self.is_shielded
    }

    /// Time elapsed since this lock was acquired.
    pub fn elapsed(&self) -> Duration {
        self.locked_at.elapsed()
    }

    /// Whether this lock has exceeded the heartbeat timeout.
    pub fn is_stale(&self) -> bool {
        self.elapsed() > HEARTBEAT_TIMEOUT
    }
}