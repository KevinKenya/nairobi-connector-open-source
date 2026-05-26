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

// File: /home/KevinKenya/nairobi-connector-open-source/crates/nairobi-hub/src/lib.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

// nairobi-open-source-release/crates/nairobi-hub/src/lib.rs
//! Nairobi Hub — The Orchestrator.
//!
//! D-Bus client proxy to Axum Refinery with Semantic Decoder.

pub mod client;
pub mod dag_parser;
pub mod decoder;
pub mod executor;
pub mod shm_subscriber;

pub use client::RefineryClient;
pub use dag_parser::{NodeType, ParsedNode};
pub use decoder::generate_correlation_report;
pub use decoder::generate_report;
pub use executor::DagExecutor;
pub use shm_subscriber::ShmSubscriber;
