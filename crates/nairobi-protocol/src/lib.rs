// File: /home/chege/nairobi-connector-open-source/crates/nairobi-protocol/src/lib.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-protocol/src/lib.rs
//! # Nairobi Protocol — The Constitution of the Nairobi Connector OS
//!
//! This crate defines the shared vocabulary for the microservice triad:
//! - **D-Bus Interface Constants** (`interface`)
//! - **GVariant-compatible Types** (`types`) — `DistilledAnalytics`
//! - **Zero-Copy Memory Infrastructure** (`mem_pipe`) — `MemoryPipe`
//! - **Shared Error Types** (`error`) — `ImperialError`
//!
//! Every component in the Empire speaks this language.

pub mod error;
pub mod interface;
pub mod mem_pipe;
pub mod types;
pub mod arena;

// ── Constitutional Re-exports ──────────────────────────────────────
pub use error::{ImperialError, ImperialResult};
pub use interface::{INTERFACE_NAME, OBJECT_PATH, SERVICE_NAME};
pub use mem_pipe::MemoryPipe;
pub use types::{CleanDataStrategy, CorrelationResult, DistilledAnalytics, FusedAnalyticsResult, SchemaInspection};
pub use arena::{ArenaHeader, PayloadType, ARENA_SERVICE_NAME, ARENA_SHM_NAME, ARENA_MAX_SIZE};
