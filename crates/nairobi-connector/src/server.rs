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

// crates/nairobi-connector/src/server.rs
// Author: Kevin Chege, Location: Nairobi, Date: 21st May 2026

//! MCP Server — bridges NeuralSession to LLM agents via the Model Context Protocol.
//!
//! Implements the `ServerHandler` trait from rmcp 1.7+ with three tools:
//! - `nairobi_get_ui_map` — returns a TOON-compressed accessibility tree
//! - `nairobi_interact` — executes semantic actions (click, focus, activate) on UI nodes
//! - `nairobi_type_text` — injects text into editable fields
//!
//! The server includes a heartbeat watcher that auto-releases the RegistryLock
//! if the stdio pipe hangs, preventing OS paralysis.

use crate::session::NeuralSession;
use crate::toon_bridge;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

/// Heartbeat check interval in milliseconds.
const HEARTBEAT_CHECK_MS: u64 = 500;

// ═══════════════════════════════════════════════════════════════════════════════
// Tool Parameter Types
// ═══════════════════════════════════════════════════════════════════════════════

/// Parameters for `nairobi_get_ui_map`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetUiMapParams {
    /// Maximum depth to traverse the accessibility tree (default: 7).
    #[serde(default = "default_max_depth")]
    pub max_depth: Option<u32>,
}

fn default_max_depth() -> Option<u32> {
    Some(7)
}

/// Parameters for `nairobi_interact`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct InteractParams {
    /// The TOON node ID to interact with (from the last `nairobi_get_ui_map` output).
    pub node_id: u32,
    /// The semantic action to execute: "click", "focus", or "activate".
    pub action: String,
}

/// Parameters for `nairobi_type_text`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TypeTextParams {
    /// The TOON node ID of the editable field.
    pub node_id: u32,
    /// The text to inject into the field (replaces existing content).
    pub text: String,
}

/// Parameters for `nairobi_find_window`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FindWindowParams {
    /// A substring to match against window titles (case-insensitive).
    pub title: String,
}

/// Structured response from `nairobi_get_ui_map`.
#[derive(Debug, Serialize)]
pub struct UiMapResponse {
    /// The TOON-compressed accessibility tree.
    pub toon: String,
    /// Number of interactive nodes found.
    pub node_count: u32,
    /// Time taken to generate the TOON in milliseconds.
    pub generation_ms: u128,
}

/// Structured response from action tools.
#[derive(Debug, Serialize)]
pub struct ActionResponse {
    /// Whether the action succeeded.
    pub success: bool,
    /// Human-readable result message.
    pub message: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// MCP Server
// ═══════════════════════════════════════════════════════════════════════════════

/// MCP server bridging AT-SPI2 semantic actions to LLM agents.
#[derive(Clone)]
pub struct NairobiServer {
    /// The AT-SPI2 session providing tree traversal and action execution.
    pub neural: Arc<NeuralSession>,
    /// Last activity timestamp for heartbeat monitoring.
    pub last_activity: Arc<RwLock<Instant>>,
}

impl NairobiServer {
    /// Create a new NairobiServer with an established NeuralSession.
    pub async fn new() -> anyhow::Result<Self> {
        let neural = Arc::new(
            NeuralSession::establish()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to establish NeuralSession: {}", e))?,
        );
        Ok(Self {
            neural,
            last_activity: Arc::new(RwLock::new(Instant::now())),
        })
    }

    /// Update the last activity timestamp.
    pub async fn touch_activity(&self) {
        let mut last = self.last_activity.write().await;
        *last = Instant::now();
    }

    /// Start the heartbeat watcher — monitors for inactivity and
    /// auto-releases RegistryLock on heartbeat timeout.
    pub fn start_heartbeat(&self) {
        let neural = self.neural.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(HEARTBEAT_CHECK_MS)).await;
                neural.check_heartbeat().await;
            }
        });
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tool Router + Handler Implementation (rmcp 1.7+)
// ═══════════════════════════════════════════════════════════════════════════════

/// Tool router — `#[tool_router]` auto-generates a `tool_router()` function
/// that maps tool names to handlers. Each `#[tool]` method becomes a callable tool.
#[tool_router]
impl NairobiServer {
    /// Find and target a window by title substring.
    ///
    /// This must be called before `nairobi_get_ui_map` to set the target window.
    /// The search is case-insensitive and matches against both app names and
    /// window frame titles.
    #[tool(description = "Find and target a window by title substring. Must be called before nairobi_get_ui_map to set which window to inspect. Case-insensitive search.")]
    async fn nairobi_find_window(
        &self,
        Parameters(params): Parameters<FindWindowParams>,
    ) -> CallToolResult {
        self.neural.touch_activity().await;

        match self.neural.find_window(&params.title).await {
            Ok((dest, path)) => {
                tracing::info!(
                    "[TOOL] nairobi_find_window: found '{}' -> dest={} path={}",
                    params.title, dest, path
                );

                let response = ActionResponse {
                    success: true,
                    message: format!(
                        "Window '{}' targeted (dest={}, path={})",
                        params.title, dest, path
                    ),
                };

                match serde_json::to_value(&response) {
                    Ok(structured) => CallToolResult::structured(structured),
                    Err(_) => CallToolResult::success(toon_bridge::wrap_text(
                        &format!("Window '{}' targeted", params.title),
                    )),
                }
            }
            Err(e) => {
                tracing::error!(
                    "[TOOL] nairobi_find_window failed: title='{}' error={}",
                    params.title, e
                );
                CallToolResult::error(toon_bridge::wrap_error(&format!(
                    "Failed to find window '{}': {}",
                    params.title, e
                )))
            }
        }
    }

    /// Get the current UI accessibility tree as a TOON-compressed map.
    ///
    /// Returns a hyper-dense Markdown representation of all interactive elements
    /// (buttons, entries, checkboxes, etc.) with sequential IDs for action targeting.
    /// Non-interactive elements (panels, fillers, separators) are stripped.
    #[tool(description = "Get the current UI accessibility tree as a TOON-compressed map. Returns a dense Markdown listing of all interactive elements with sequential IDs for action targeting.")]
    async fn nairobi_get_ui_map(
        &self,
        Parameters(params): Parameters<GetUiMapParams>,
    ) -> CallToolResult {
        self.neural.touch_activity().await;
        let max_depth = params.max_depth.unwrap_or(7);

        match self.neural.get_ui_map(max_depth).await {
            Ok((toon, node_count, generation_ms)) => {
                tracing::info!(
                    "[TOOL] nairobi_get_ui_map: {} nodes in {}ms (depth={})",
                    node_count, generation_ms, max_depth
                );

                let response = UiMapResponse {
                    toon: toon.clone(),
                    node_count,
                    generation_ms,
                };

                // Return structured result with TOON text as primary content
                match serde_json::to_value(&response) {
                    Ok(structured) => CallToolResult::structured(structured),
                    Err(_) => CallToolResult::success(toon_bridge::wrap_toon(&toon)),
                }
            }
            Err(e) => {
                tracing::error!("[TOOL] nairobi_get_ui_map failed: {}", e);
                CallToolResult::error(toon_bridge::wrap_error(&format!(
                    "Failed to get UI map: {}",
                    e
                )))
            }
        }
    }

    /// Execute a semantic action on a UI element identified by its TOON node ID.
    ///
    /// Supported actions:
    /// - "click" or "activate": Triggers the element's primary action
    /// - "focus": Sets keyboard focus to the element
    #[tool(description = "Execute a semantic action on a UI element. Actions: 'click', 'activate', 'focus'. The node_id must come from the last nairobi_get_ui_map call.")]
    async fn nairobi_interact(
        &self,
        Parameters(params): Parameters<InteractParams>,
    ) -> CallToolResult {
        self.neural.touch_activity().await;

        // Validate action name
        let valid_actions = ["click", "activate", "focus"];
        if !valid_actions.contains(&params.action.as_str()) {
            return CallToolResult::error(toon_bridge::wrap_error(&format!(
                "Invalid action '{}'. Supported: click, activate, focus",
                params.action
            )));
        }

        match self
            .neural
            .interact_by_id(params.node_id, &params.action)
            .await
        {
            Ok(message) => {
                tracing::info!(
                    "[TOOL] nairobi_interact: node={} action={} -> {}",
                    params.node_id,
                    params.action,
                    message
                );

                let response = ActionResponse {
                    success: true,
                    message,
                };

                match serde_json::to_value(&response) {
                    Ok(structured) => CallToolResult::structured(structured),
                    Err(_) => CallToolResult::success(toon_bridge::wrap_text(&format!(
                        "Action '{}' executed on node {}",
                        params.action, params.node_id
                    ))),
                }
            }
            Err(e) => {
                tracing::error!(
                    "[TOOL] nairobi_interact failed: node={} action={} error={}",
                    params.node_id,
                    params.action,
                    e
                );
                CallToolResult::error(toon_bridge::wrap_error(&format!(
                    "Interact failed (node={}, action={}): {}",
                    params.node_id, params.action, e
                )))
            }
        }
    }

    /// Inject text into an editable field identified by its TOON node ID.
    ///
    /// This atomically replaces the entire text content of an Entry, TextArea,
    /// or other EditableText element.
    #[tool(description = "Inject text into an editable UI field. Atomically replaces all text in the element identified by node_id. The node_id must come from the last nairobi_get_ui_map call.")]
    async fn nairobi_type_text(
        &self,
        Parameters(params): Parameters<TypeTextParams>,
    ) -> CallToolResult {
        self.neural.touch_activity().await;

        match self
            .neural
            .type_text_by_id(params.node_id, &params.text)
            .await
        {
            Ok(message) => {
                tracing::info!(
                    "[TOOL] nairobi_type_text: node={} text_len={} -> {}",
                    params.node_id,
                    params.text.len(),
                    message
                );

                let response = ActionResponse {
                    success: true,
                    message,
                };

                match serde_json::to_value(&response) {
                    Ok(structured) => CallToolResult::structured(structured),
                    Err(_) => CallToolResult::success(toon_bridge::wrap_text("Text injected")),
                }
            }
            Err(e) => {
                tracing::error!(
                    "[TOOL] nairobi_type_text failed: node={} error={}",
                    params.node_id,
                    e
                );
                CallToolResult::error(toon_bridge::wrap_error(&format!(
                    "Type text failed (node={}): {}",
                    params.node_id, e
                )))
            }
        }
    }
}

/// `#[tool_handler]` auto-generates `list_tools()`, `call_tool()`, `get_tool()`,
/// and `get_info()` on the `ServerHandler` impl using the tool router above.
#[tool_handler(
    name = "nairobi-connector",
    version = "0.4.0",
    instructions = "Nairobi Connector — Computer Use without pixels.\n\nUse nairobi_get_ui_map to see all interactive UI elements as a TOON tree.\nEach element has a sequential [ID: N] you can reference in actions.\nUse nairobi_interact to click/focus elements.\nUse nairobi_type_text to type into editable fields.\n\nAlways call nairobi_get_ui_map first to get fresh IDs before interacting."
)]
impl ServerHandler for NairobiServer {}