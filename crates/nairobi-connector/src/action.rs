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

// crates/nairobi-connector/src/action.rs
// Author: Kevin Chege, Location: Nairobi, Date: 11th May 2026

//! Semantic Actions — Coordinate-free widget manipulation via AT-SPI2.

use crate::error::{NeuralError, Result};
use crate::engine::DFSEngine;
use atspi::{
    proxy::{
        action::ActionProxy,
        component::ComponentProxy,
        editable_text::EditableTextProxy,
        selection::SelectionProxy,
        value::ValueProxy,
    },
    Interface, State,
};
use std::time::Duration;
use tokio::time::timeout;
use zbus::Connection;

const ACTION_TIMEOUT: Duration = Duration::from_millis(2000);

#[derive(Debug, Clone)]
pub enum SemanticAction {
    Activate,
    SetText(String),
    SetValue(f64),
    SelectIndex(i32),
    GrabFocus,
}

pub async fn do_action(conn: &Connection, dest: &str, path: &str, index: i32) -> Result<bool> {
    let proxy = build_action_proxy(conn, dest, path).await?;
    let result = timeout(ACTION_TIMEOUT, proxy.do_action(index))
        .await
        .map_err(|_| NeuralError::Timeout(format!("DoAction({}) on {}", index, path)))?
        .map_err(|e| NeuralError::ActionFailed(format!("DoAction({}) failed: {}", index, e)))?;
    Ok(result)
}

pub async fn do_action_by_name(
    conn: &Connection, dest: &str, path: &str, name: &str,
) -> Result<bool> {
    let proxy = build_action_proxy(conn, dest, path).await?;
    let actions = timeout(ACTION_TIMEOUT, proxy.get_actions())
        .await
        .map_err(|_| NeuralError::Timeout("GetActions".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("GetActions failed: {}", e)))?;
    let lower_name = name.to_lowercase();
    for (i, action) in actions.iter().enumerate() {
        if action.name.to_lowercase() == lower_name {
            return do_action(conn, dest, path, i as i32).await;
        }
    }
    Err(NeuralError::ActionFailed(format!(
        "No action named '{}' found",
        name
    )))
}

pub async fn get_available_actions(
    conn: &Connection, dest: &str, path: &str,
) -> Vec<(String, String)> {
    let proxy = match build_action_proxy(conn, dest, path).await {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };
    match timeout(ACTION_TIMEOUT, proxy.get_actions()).await {
        Ok(Ok(actions)) => actions
            .into_iter()
            .map(|a| (a.name.clone(), a.description.clone()))
            .collect(),
        _ => Vec::new(),
    }
}

pub async fn set_text(conn: &Connection, dest: &str, path: &str, text: &str) -> Result<()> {
    let proxy = build_editable_text_proxy(conn, dest, path).await?;
    timeout(ACTION_TIMEOUT, proxy.set_text_contents(text))
        .await
        .map_err(|_| NeuralError::Timeout("SetTextContents".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("SetTextContents failed: {}", e)))?;
    Ok(())
}

pub async fn set_value(conn: &Connection, dest: &str, path: &str, value: f64) -> Result<()> {
    let proxy = build_value_proxy(conn, dest, path).await?;
    timeout(ACTION_TIMEOUT, proxy.set_current_value(value))
        .await
        .map_err(|_| NeuralError::Timeout("SetCurrentValue".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("SetCurrentValue failed: {}", e)))?;
    Ok(())
}

pub async fn get_value(conn: &Connection, dest: &str, path: &str) -> Result<f64> {
    let proxy = build_value_proxy(conn, dest, path).await?;
    let value: f64 = timeout(ACTION_TIMEOUT, proxy.current_value())
        .await
        .map_err(|_| NeuralError::Timeout("GetCurrentValue".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("GetCurrentValue failed: {}", e)))?;
    Ok(value)
}

pub async fn get_value_range(conn: &Connection, dest: &str, path: &str) -> Result<(f64, f64)> {
    let proxy = build_value_proxy(conn, dest, path).await?;
    let min: f64 = timeout(ACTION_TIMEOUT, proxy.minimum_value())
        .await
        .map_err(|_| NeuralError::Timeout("GetMinimumValue".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("GetMinimumValue failed: {}", e)))?;
    let max: f64 = timeout(ACTION_TIMEOUT, proxy.maximum_value())
        .await
        .map_err(|_| NeuralError::Timeout("GetMaximumValue".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("GetMaximumValue failed: {}", e)))?;
    Ok((min, max))
}

pub async fn is_enabled(conn: &Connection, dest: &str, path: &str) -> bool {
    let proxy = match DFSEngine::timeout_proxy_build(conn, dest, path).await {
        Ok(p) => p,
        Err(_) => return false,
    };
    match timeout(ACTION_TIMEOUT, proxy.get_state()).await {
        Ok(Ok(states)) => states.contains(State::Sensitive),
        _ => false,
    }
}

pub async fn is_focused(conn: &Connection, dest: &str, path: &str) -> bool {
    let proxy = match DFSEngine::timeout_proxy_build(conn, dest, path).await {
        Ok(p) => p,
        Err(_) => return false,
    };
    match timeout(ACTION_TIMEOUT, proxy.get_state()).await {
        Ok(Ok(states)) => states.contains(State::Focused),
        _ => false,
    }
}

pub async fn grab_focus(conn: &Connection, dest: &str, path: &str) -> Result<bool> {
    let proxy = build_component_proxy(conn, dest, path).await?;
    let result: bool = timeout(ACTION_TIMEOUT, proxy.grab_focus())
        .await
        .map_err(|_| NeuralError::Timeout("GrabFocus".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("GrabFocus failed: {}", e)))?;
    Ok(result)
}

pub async fn execute_semantic_action(
    conn: &Connection, dest: &str, path: &str, action: SemanticAction,
) -> Result<()> {
    match action {
        SemanticAction::Activate => execute_activate(conn, dest, path).await,
        SemanticAction::SetText(text) => set_text(conn, dest, path, &text).await,
        SemanticAction::SetValue(normalized) => {
            execute_set_value(conn, dest, path, normalized).await
        }
        SemanticAction::SelectIndex(index) => {
            execute_select_index(conn, dest, path, index).await
        }
        SemanticAction::GrabFocus => {
            grab_focus(conn, dest, path).await?;
            Ok(())
        }
    }
}

async fn execute_activate(conn: &Connection, dest: &str, path: &str) -> Result<()> {
    if !is_enabled(conn, dest, path).await {
        return Err(NeuralError::ElementDisabled(format!(
            "Element at {} is not enabled",
            path
        )));
    }
    let proxy = build_action_proxy(conn, dest, path).await?;
    let actions = timeout(ACTION_TIMEOUT, proxy.get_actions())
        .await
        .map_err(|_| NeuralError::Timeout("GetActions".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("GetActions failed: {}", e)))?;
    if actions.is_empty() {
        return Err(NeuralError::ActionFailed(
            "No actions available".to_string(),
        ));
    }
    let result = timeout(ACTION_TIMEOUT, proxy.do_action(0))
        .await
        .map_err(|_| NeuralError::Timeout("DoAction(0)".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("DoAction(0) failed: {}", e)))?;
    if !result {
        return Err(NeuralError::ActionFailed(
            "Activation returned false".to_string(),
        ));
    }
    Ok(())
}

async fn execute_set_value(
    conn: &Connection, dest: &str, path: &str, normalized_value: f64,
) -> Result<()> {
    let proxy = build_value_proxy(conn, dest, path).await?;
    let min: f64 = timeout(ACTION_TIMEOUT, proxy.minimum_value())
        .await
        .map_err(|_| NeuralError::Timeout("Get MinimumValue".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("Get MinimumValue failed: {}", e)))?;
    let max: f64 = timeout(ACTION_TIMEOUT, proxy.maximum_value())
        .await
        .map_err(|_| NeuralError::Timeout("Get MaximumValue".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("Get MaximumValue failed: {}", e)))?;
    let target = (min + normalized_value * (max - min)).clamp(min, max);
    timeout(ACTION_TIMEOUT, proxy.set_current_value(target))
        .await
        .map_err(|_| NeuralError::Timeout("Set CurrentValue".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("Set CurrentValue failed: {}", e)))?;
    Ok(())
}

async fn execute_select_index(
    conn: &Connection, dest: &str, path: &str, index: i32,
) -> Result<()> {
    let proxy = build_selection_proxy(conn, dest, path).await?;
    let result: bool = timeout(ACTION_TIMEOUT, proxy.select_child(index))
        .await
        .map_err(|_| NeuralError::Timeout("SelectChild".to_string()))?
        .map_err(|e| NeuralError::ActionFailed(format!("SelectChild failed: {}", e)))?;
    if !result {
        return Err(NeuralError::ActionFailed(format!(
            "SelectChild({}) returned false",
            index
        )));
    }
    Ok(())
}

// ─── Proxy Builders ────────────────────────────────────────────
// zbus 5.x API: builder(conn) → Builder, .destination(dest) → Result<Builder>,
// .path(path) → Result<Builder>, .build() → async Result<Proxy>

async fn build_action_proxy<'a>(
    conn: &'a Connection, dest: &'a str, path: &'a str,
) -> Result<ActionProxy<'a>> {
    let proxy = DFSEngine::timeout_proxy_build(conn, dest, path).await?;
    let interfaces = timeout(ACTION_TIMEOUT, proxy.get_interfaces())
        .await
        .map_err(|_| NeuralError::Timeout("GetInterfaces".to_string()))?
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?;
    if !interfaces.contains(Interface::Action) {
        return Err(NeuralError::InterfaceNotSupported("Action".to_string()));
    }
    ActionProxy::builder(proxy.inner().connection())
        .destination(dest)
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?
        .path(path)
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?
        .build()
        .await
        .map_err(|e| NeuralError::ZbusError(format!("ActionProxy build: {}", e)))
}

async fn build_editable_text_proxy<'a>(
    conn: &'a Connection, dest: &'a str, path: &'a str,
) -> Result<EditableTextProxy<'a>> {
    let proxy = DFSEngine::timeout_proxy_build(conn, dest, path).await?;
    let interfaces = timeout(ACTION_TIMEOUT, proxy.get_interfaces())
        .await
        .map_err(|_| NeuralError::Timeout("GetInterfaces".to_string()))?
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?;
    if !interfaces.contains(Interface::EditableText) {
        return Err(NeuralError::InterfaceNotSupported(
            "EditableText".to_string(),
        ));
    }
    EditableTextProxy::builder(proxy.inner().connection())
        .destination(dest)
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?
        .path(path)
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?
        .build()
        .await
        .map_err(|e| NeuralError::ZbusError(format!("EditableTextProxy build: {}", e)))
}

async fn build_value_proxy<'a>(
    conn: &'a Connection, dest: &'a str, path: &'a str,
) -> Result<ValueProxy<'a>> {
    let proxy = DFSEngine::timeout_proxy_build(conn, dest, path).await?;
    let interfaces = timeout(ACTION_TIMEOUT, proxy.get_interfaces())
        .await
        .map_err(|_| NeuralError::Timeout("GetInterfaces".to_string()))?
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?;
    if !interfaces.contains(Interface::Value) {
        return Err(NeuralError::InterfaceNotSupported("Value".to_string()));
    }
    ValueProxy::builder(proxy.inner().connection())
        .destination(dest)
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?
        .path(path)
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?
        .build()
        .await
        .map_err(|e| NeuralError::ZbusError(format!("ValueProxy build: {}", e)))
}

async fn build_selection_proxy<'a>(
    conn: &'a Connection, dest: &'a str, path: &'a str,
) -> Result<SelectionProxy<'a>> {
    let proxy = DFSEngine::timeout_proxy_build(conn, dest, path).await?;
    let interfaces = timeout(ACTION_TIMEOUT, proxy.get_interfaces())
        .await
        .map_err(|_| NeuralError::Timeout("GetInterfaces".to_string()))?
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?;
    if !interfaces.contains(Interface::Selection) {
        return Err(NeuralError::InterfaceNotSupported(
            "Selection".to_string(),
        ));
    }
    SelectionProxy::builder(proxy.inner().connection())
        .destination(dest)
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?
        .path(path)
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?
        .build()
        .await
        .map_err(|e| NeuralError::ZbusError(format!("SelectionProxy build: {}", e)))
}

async fn build_component_proxy<'a>(
    conn: &'a Connection, dest: &'a str, path: &'a str,
) -> Result<ComponentProxy<'a>> {
    let proxy = DFSEngine::timeout_proxy_build(conn, dest, path).await?;
    let interfaces = timeout(ACTION_TIMEOUT, proxy.get_interfaces())
        .await
        .map_err(|_| NeuralError::Timeout("GetInterfaces".to_string()))?
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?;
    if !interfaces.contains(Interface::Component) {
        return Err(NeuralError::InterfaceNotSupported(
            "Component".to_string(),
        ));
    }
    ComponentProxy::builder(proxy.inner().connection())
        .destination(dest)
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?
        .path(path)
        .map_err(|e| NeuralError::ZbusError(e.to_string()))?
        .build()
        .await
        .map_err(|e| NeuralError::ZbusError(format!("ComponentProxy build: {}", e)))
}