// File: /home/KevinKenya/nairobi-connector-open-source/crates/nairobi-hub/src/lib.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-hub/src/lib.rs
//! Nairobi Hub — The Orchestrator.
//!
//! D-Bus client proxy to Axum Refinery with Semantic Decoder.

pub mod client;
pub mod decoder;
pub mod shm_subscriber;

pub use client::RefineryClient;
pub use decoder::generate_correlation_report;
pub use decoder::generate_report;
pub use shm_subscriber::ShmSubscriber;
