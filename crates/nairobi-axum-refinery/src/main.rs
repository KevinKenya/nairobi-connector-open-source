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

// File: crates/nairobi-axum-refinery/src/main.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

// nairobi-open-source-release/crates/nairobi-axum-refinery/src/main.rs
use nairobi_axum_refinery::dbus_service::AxumRefineryService;
use nairobi_protocol::{OBJECT_PATH, SERVICE_NAME};
use tracing::info;
use tracing_subscriber::EnvFilter;
use zbus::ConnectionBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize High-Fidelity Tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    info!("[NAIROBI-AXUM-REFINERY] Starting Heavy Iron Engine...");

    // 2. Initialize Service State (1GB Buffer capacity)
    let buffer_size = 1024 * 1024 * 1024; // 1GB
    let service = AxumRefineryService::new(buffer_size)?;

    // 3. Request D-Bus name and serve interface
    let _conn = ConnectionBuilder::session()?
        .name(SERVICE_NAME)?
        .serve_at(OBJECT_PATH, service)?
        .build()
        .await?;

    info!(
        "[NAIROBI-AXUM-REFINERY] Service org.nairobi.AxumRefinery1 is LIVE at {}",
        OBJECT_PATH
    );

    // 4. Block on pending
    std::future::pending::<()>().await;

    Ok(())
}
