// File: /home/chege/nairobi-connector-open-source/crates/nairobi-protocol/src/error.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-protocol/src/error.rs
//! Shared error types for the Nairobi microservice stack.
//!
//! `ImperialError` is the constitutional error type — every crate in the
//! triad (nairobi-protocol, axum-refinery, nairobi-hub) converges on
//! these variants.

/// The constitutional error type for all Nairobi microservices.
#[derive(Debug, thiserror::Error)]
pub enum ImperialError {
    /// D-Bus communication failure.
    #[error("D-Bus error: {0}")]
    Dbus(#[from] zbus::Error),

    /// Data ingestion failure (io_uring, memfd, mmap).
    #[error("Ingestion failed: {0}")]
    Ingestion(String),

    /// Analytical processing failure (Polars, SIMD).
    #[error("Analysis failed: {0}")]
    Analysis(String),

    /// Kernel-level resource exhaustion or state corruption.
    #[error("CRITICAL: Systemic seizure: {0}")]
    SystemicSeizure(String),

    /// GVariant serialization/deserialization failure.
    #[error("GVariant codec error: {0}")]
    Codec(String),

    /// Rendering failure in Lagos Vision (egui/wgpu).
    #[error("Lagos rendering error: {0}")]
    LagosRendering(String),
}

/// Result type alias using `ImperialError`.
pub type ImperialResult<T> = std::result::Result<T, ImperialError>;
