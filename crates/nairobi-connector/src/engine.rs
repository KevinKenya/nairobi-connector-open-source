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

// crates/nairobi-connector/src/engine.rs
// Author: Kevin Chege, Location: Nairobi, Date: 11th May 2026

//! DFS Tree Traversal Engine — AT-SPI2 accessibility tree DFS with zbus 5.x / atspi 0.30.

use crate::error::{NeuralError, Result};
use atspi::{
    object_ref::ObjectRefOwned,
    proxy::{accessible::AccessibleProxy, action::ActionProxy, text::TextProxy},
    Role, State,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use zbus::Connection;

const MAX_PARALLELISM: usize = 32;
const DFS_TIMEOUT: Duration = Duration::from_millis(2000);
const MAX_DEPTH: u32 = 20;

#[derive(Debug, Clone)]
pub struct UISnapshot {
    pub role: Role,
    pub name: String,
    pub actions: Vec<String>,
    pub children: Vec<UISnapshot>,
    pub depth: u32,
    /// D-Bus destination (bus name) for this node.
    pub destination: String,
    /// D-Bus object path for this node.
    pub object_path: String,
}

pub struct DFSEngine;

impl DFSEngine {
    pub async fn timeout_proxy_build<'a>(
        conn: &'a Connection,
        dest: &'a str,
        path: &'a str,
    ) -> Result<AccessibleProxy<'a>> {
        let proxy = AccessibleProxy::builder(conn)
            .destination(dest)
            .map_err(|e| NeuralError::ZbusError(e.to_string()))?
            .path(path)
            .map_err(|e| NeuralError::ZbusError(e.to_string()))?
            .build()
            .await
            .map_err(|e| NeuralError::ZbusError(e.to_string()))?;
        Ok(proxy)
    }

    pub fn get_proxy_info(proxy: &AccessibleProxy<'_>) -> (String, String) {
        let dest = proxy.inner().destination().to_string();
        let path = proxy.inner().path().to_string();
        (dest, path)
    }

    pub async fn get_name(proxy: &AccessibleProxy<'_>) -> String {
        timeout(DFS_TIMEOUT, proxy.name())
            .await.unwrap_or_else(|_| Ok(String::new())).unwrap_or_default()
    }

    pub async fn get_description(proxy: &AccessibleProxy<'_>) -> String {
        timeout(DFS_TIMEOUT, proxy.description())
            .await.unwrap_or_else(|_| Ok(String::new())).unwrap_or_default()
    }

    pub async fn get_role(proxy: &AccessibleProxy<'_>) -> Role {
        timeout(DFS_TIMEOUT, proxy.get_role())
            .await.unwrap_or_else(|_| Ok(Role::Unknown)).unwrap_or(Role::Unknown)
    }

    pub async fn get_children(proxy: &AccessibleProxy<'_>) -> Vec<ObjectRefOwned> {
        timeout(DFS_TIMEOUT, proxy.get_children())
            .await.unwrap_or_else(|_| Ok(Vec::new())).unwrap_or_default()
    }

    pub async fn get_state_set(proxy: &AccessibleProxy<'_>) -> atspi::StateSet {
        timeout(DFS_TIMEOUT, proxy.get_state())
            .await.unwrap_or_else(|_| Ok(atspi::StateSet::empty())).unwrap_or_else(|_| atspi::StateSet::empty())
    }

    pub async fn get_interfaces(proxy: &AccessibleProxy<'_>) -> atspi::InterfaceSet {
        timeout(DFS_TIMEOUT, proxy.get_interfaces())
            .await.unwrap_or_else(|_| Ok(atspi::InterfaceSet::empty())).unwrap_or_else(|_| atspi::InterfaceSet::empty())
    }

    pub fn is_semantic_role(role: Role) -> bool {
        matches!(role,
            Role::Button | Role::ToggleButton | Role::Link | Role::Entry
            | Role::Text | Role::DocumentText | Role::Terminal
            | Role::Heading | Role::MenuItem | Role::Alert | Role::Dialog
            | Role::Frame | Role::PageTab | Role::Label
            | Role::Menu | Role::CheckBox | Role::RadioButton | Role::SpinButton
            | Role::Slider | Role::ComboBox | Role::Table | Role::TableCell
            | Role::Tree | Role::TreeItem | Role::ScrollBar | Role::ProgressBar
        )
    }

    pub fn is_interactive_role(role: Role) -> bool {
        matches!(role,
            Role::Button | Role::ToggleButton | Role::MenuItem
            | Role::CheckMenuItem | Role::RadioMenuItem | Role::Entry
            | Role::PasswordText | Role::SpinButton | Role::Slider
            | Role::ComboBox | Role::ListItem | Role::PageTab
            | Role::PageTabList | Role::Link | Role::TreeItem
            | Role::CheckBox | Role::RadioButton | Role::DocumentText | Role::Terminal
        )
    }

    pub fn clean_label(label: &str) -> String {
        let mut t = label.trim().to_string();
        if t.is_empty() { return String::new(); }
        if let Some(idx) = t.rfind(" (") {
            let s = &t[idx..];
            if s.contains(';') || s.contains('+') || s.contains("<Alt>") || s.contains("<Ctrl>") || s.len() < 10 {
                t = t[..idx].trim().to_string();
            }
        }
        if let Some(idx) = t.find(';') { t = t[..idx].trim().to_string(); }
        if (t.starts_with('_') || t.starts_with('&')) && t.len() > 1 {
            t = t[1..].to_string();
        }
        t = t.replace('&', "").replace('_', "");
        if t.is_empty() || (t.len() == 1 && !t.chars().next().unwrap().is_alphabetic()) {
            return String::new();
        }
        let l = t.to_lowercase();
        if matches!(l.as_str(), "click" | "press" | "activate" | "unnamed" | "action") {
            return String::new();
        }
        t
    }

    pub async fn get_text_content(conn: &Connection, dest: &str, path: &str) -> String {
        let tp = match TextProxy::builder(conn)
            .destination(dest)
            .and_then(|b| b.path(path))
        {
            Ok(b) => match timeout(DFS_TIMEOUT, b.build()).await {
                Ok(Ok(p)) => p,
                _ => return String::new(),
            },
            Err(_) => return String::new(),
        };
        timeout(DFS_TIMEOUT, tp.get_text(0, 2048))
            .await.unwrap_or(Ok(String::new())).unwrap_or_default()
    }

    pub async fn resolve_label(proxy: &AccessibleProxy<'_>) -> String {
        let (name, desc) = tokio::join!(Self::get_name(proxy), Self::get_description(proxy));
        let cleaned = Self::clean_label(&name);
        if !cleaned.is_empty() { return cleaned; }
        let cleaned = Self::clean_label(&desc);
        if !cleaned.is_empty() { return cleaned; }
        let role = Self::get_role(proxy).await;
        if Self::is_semantic_role(role) {
            let (dest, pth) = Self::get_proxy_info(proxy);
            let text = Self::get_text_content(proxy.inner().connection(), &dest, &pth).await;
            let cleaned = Self::clean_label(&text);
            if !cleaned.is_empty() { return cleaned; }
        }
        String::new()
    }

    pub async fn get_actions(conn: &Connection, dest: &str, path: &str) -> Vec<String> {
        let ap = match ActionProxy::builder(conn)
            .destination(dest)
            .and_then(|b| b.path(path))
        {
            Ok(b) => match timeout(DFS_TIMEOUT, b.build()).await {
                Ok(Ok(p)) => p,
                _ => return Vec::new(),
            },
            Err(_) => return Vec::new(),
        };
        if let Ok(Ok(actions)) = timeout(DFS_TIMEOUT, ap.get_actions()).await {
            return actions.into_iter().filter_map(|a| {
                let c = Self::clean_label(&a.name);
                if !c.is_empty() && c.chars().any(|ch| ch.is_alphabetic()) { Some(c) } else { None }
            }).collect();
        }
        Vec::new()
    }

    pub async fn is_visible(proxy: &AccessibleProxy<'_>) -> bool {
        Self::get_state_set(proxy).await.contains(State::Visible)
    }

    pub async fn capture_snapshot(proxy: AccessibleProxy<'_>) -> UISnapshot {
        let (dest, pth) = Self::get_proxy_info(&proxy);
        let semaphore = Arc::new(Semaphore::new(MAX_PARALLELISM));
        Self::capture_recursive(proxy.inner().connection().clone(), dest, pth, semaphore, 0).await
    }

    #[async_recursion::async_recursion]
    async fn capture_recursive(
        conn: Connection, dest: String, path: String, semaphore: Arc<Semaphore>, depth: u32,
    ) -> UISnapshot {
        if depth > MAX_DEPTH {
            return UISnapshot { role: Role::Unknown, name: String::new(), actions: vec![], children: vec![], depth, destination: String::new(), object_path: String::new() };
        }
        let (role, name, actions, children_raw) = {
            let _permit = match semaphore.acquire().await {
                Ok(p) => p,
                Err(_) => return UISnapshot { role: Role::Unknown, name: String::new(), actions: vec![], children: vec![], depth, destination: dest, object_path: path },
            };
            let proxy = match Self::timeout_proxy_build(&conn, &dest, &path).await {
                Ok(p) => p, Err(_) => return UISnapshot { role: Role::Unknown, name: String::new(), actions: vec![], children: vec![], depth, destination: dest, object_path: path },
            };
            let role = Self::get_role(&proxy).await;
            let is_semantic = Self::is_semantic_role(role);
            let visible = Self::is_visible(&proxy).await || dest == "org.a11y.atspi.Registry" || role == Role::Application;
            if !visible {
                return UISnapshot { role: Role::Unknown, name: String::new(), actions: vec![], children: vec![], depth, destination: dest, object_path: path };
            }
            let name = if is_semantic { Self::resolve_label(&proxy).await } else { Self::get_name(&proxy).await };
            let (dest_str, pth_str) = Self::get_proxy_info(&proxy);
            let actions = if is_semantic { Self::get_actions(&conn, &dest_str, &pth_str).await } else { vec![] };
            let children_raw = if depth < MAX_DEPTH { Self::get_children(&proxy).await } else { vec![] };
            (role, name, actions, children_raw)
        };
        if children_raw.is_empty() {
            return UISnapshot { role, name, actions, children: vec![], depth, destination: dest, object_path: path };
        }
        let mut futures = vec![];
        for child in children_raw {
            let nd = child.name_as_str().unwrap_or_default().to_string();
            let np = child.path_as_str().to_string();
            let s = semaphore.clone();
            let c = conn.clone();
            futures.push(tokio::spawn(async move {
                Self::capture_recursive(c, nd, np, s, depth + 1).await
            }));
        }
        let results = futures_util::future::join_all(futures).await;
        let children: Vec<UISnapshot> = results.into_iter().filter_map(|r| r.ok()).filter(|s| s.role != Role::Unknown).collect();
        UISnapshot { role, name, actions, children, depth, destination: dest, object_path: path }
    }
}