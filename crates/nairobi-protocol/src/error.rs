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

// File: /home/KevinKenya/nairobi-connector-open-source/crates/nairobi-protocol/src/error.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

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
