// File: /home/chege/nairobi-connector-open-source/crates/nairobi-axum-refinery/src/main.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-axum-refinery/src/main.rs
use nairobi_axum_refinery::dbus_service::AxumRefineryService;
use nairobi_protocol::{OBJECT_PATH, SERVICE_NAME};
use tracing::{error, info};
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
