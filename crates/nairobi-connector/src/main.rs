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

// crates/nairobi-connector/src/main.rs
// Author: Kevin Chege, Location: Nairobi, Date: 21st May 2026

//! Nairobi Connector — MCP server entry point.
//!
//! Bridges AT-SPI2 semantic actions (The Action Engine) to LLM agents via the
//! Model Context Protocol (MCP) over stdio transport.
//!
//! Usage:
//!   nairobi-connector
//!
//! The server communicates via stdin/stdout using JSON-RPC 2.0.
//! All human-readable logs go to stderr to avoid polluting the MCP protocol stream.

use nairobi_connector::server::NairobiServer;
use rmcp::ServiceExt;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Redirect all logs to stderr so stdout stays clean for MCP protocol
    fmt::Subscriber::builder()
        .with_writer(std::io::stderr)
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("nairobi_connector=info".parse()?),
        )
        .init();

    tracing::info!("[MAIN] Nairobi Connector MCP server starting...");
    tracing::info!("[MAIN] Transport: stdio (stdin/stdout)");

    // Establish AT-SPI2 session and create MCP server
    let server = NairobiServer::new().await?;

    // Start heartbeat watcher — auto-releases RegistryLock on pipe hang
    server.start_heartbeat();
    tracing::info!(
        "[HEARTBEAT] Watcher active — timeout: {}s",
        nairobi_connector::safety::HEARTBEAT_TIMEOUT.as_secs()
    );

    tracing::info!("[MAIN] NeuralSession established — AT-SPI2 bridge ready");
    tracing::info!("[MAIN] Serving MCP over stdio — awaiting client connection...");

    // Serve the MCP server over stdio transport (stdin/stdout)
    // rmcp::transport::io::stdio() returns (tokio::io::Stdin, tokio::io::Stdout)
    let transport = rmcp::transport::io::stdio();

    // serve_server handles the full MCP lifecycle:
    // 1. Receives InitializeRequest from client
    // 2. Responds with ServerInfo (capabilities, tools list)
    // 3. Enters main request loop (list_tools, call_tool, ping, etc.)
    // 4. Shuts down on EOF/pipe close
    let running = server.serve(transport).await
        .inspect_err(|e| tracing::error!("[MAIN] MCP server failed to start: {}", e))?;

    tracing::info!("[MAIN] MCP server initialized — serving tools");

    // Wait for the server to complete (pipe close / client disconnect)
    running.waiting().await?;

    tracing::info!("[MAIN] MCP server shut down gracefully");
    Ok(())
}