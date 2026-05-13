// File: /home/chege/nairobi-connector-open-source/crates/nairobi-protocol/src/interface.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

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

