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

// File: crates/nairobi-protocol/src/interface.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

// nairobi-open-source-release/crates/nairobi-protocol/src/interface.rs
//! D-Bus interface constants for the Axum Refinery service.
//!
//! These define the well-known names and object paths that constitute
//! the GVariant handshake protocol between the Hub and the Refinery.

/// The D-Bus interface name exposed by the Axum Refinery daemon.
///
/// Methods on this interface:
/// - `Ingest(s file_path, s delimiter, s encoding) → h memfd_handle`
/// - `Analyze(h memfd_handle, s query) → v gvariant_result`
/// - `InspectSchema(h memfd_handle) → v schema_inspection`
/// - `CleanData(h memfd_handle, a(sss) strategies) → h memfd_handle`
/// - `SqlQuery(h memfd_handle, s query) → h memfd_handle`
/// - `Correlation(h memfd_handle, s query) → (dd)`
/// - `CrunchAndCorrelate(h memfd_handle, s column, s corr_columns) → v fused_result`
/// - `IngestCrunchCorrelate(s file_path, s delimiter, s encoding, s column, s corr_columns) → v fused_result`
///
/// GVariant signature for `Analyze` return value `v`: `(tdddddddddhas)`
/// GVariant signature for fused results `v`: `(tdddddddddddasdd)`
pub const INTERFACE_NAME: &str = "org.nairobi.NairobiAxumRefinery1";

/// The well-known D-Bus service name that the Refinery daemon registers.
pub const SERVICE_NAME: &str = "org.nairobi.NairobiAxumRefinery1";

/// The object path at which the Refinery interface is mounted.
pub const OBJECT_PATH: &str = "/org/nairobi/NairobiAxumRefinery1";

/// The D-Bus interface name for the Nairobi Hub's ExecuteDag endpoint.
///
/// Methods on this interface:
/// - `ExecuteDag(ay dag_bytes) → s` - Executes a compiled DAG, returns status/result
pub const HUB_INTERFACE_NAME: &str = "org.nairobi.NairobiHub1";
pub const HUB_SERVICE_NAME: &str = "org.nairobi.NairobiHub1";
pub const HUB_OBJECT_PATH: &str = "/org/nairobi/NairobiHub1";

