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

// File: /home/KevinKenya/nairobi-connector-open-source/crates/nairobi-protocol/src/lib.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

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
