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

// File: crates/nairobi-axum-refinery/src/dbus_service.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

// nairobi-open-source-release/crates/nairobi-axum-refinery/src/dbus_service.rs
//
// v0.1.2 REFIT: D-Bus is now the Control Plane only.
// The Data Plane uses iceoryx2 shared memory arenas.
// Methods return "SHM_READY" when data is available in the arena,
// or fall back to GVariant JSON payloads if iceoryx2 is unavailable.

use crate::analyze::AnalyzeEngine;
use crate::ingest::DiracEngine;
use crate::shm_publisher::ShmPublisher;
use nairobi_protocol::{
    CleanDataStrategy, ImperialError, PayloadType, SchemaInspection,
};
use tracing::{info, warn, error};
use zbus::dbus_interface;
use zbus::zvariant::OwnedFd;

pub struct AxumRefineryService {
    ingest_engine: DiracEngine,
    analyze_engine: AnalyzeEngine,
    /// iceoryx2 shared memory publisher (Data Plane).
    /// `None` if iceoryx2 initialization failed — graceful degradation to D-Bus GVariant.
    shm_publisher: Option<ShmPublisher>,
}

impl AxumRefineryService {
    pub fn new(buffer_size: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let ingest_engine = DiracEngine::new(buffer_size)?;
        let analyze_engine = AnalyzeEngine::new()?;

        // Attempt iceoryx2 Data Plane initialization
        let shm_publisher = match ShmPublisher::new() {
            Ok(publisher) => {
                info!("[ICEORYX2] Data Plane initialized. D-Bus relegated to Control Plane.");
                Some(publisher)
            }
            Err(e) => {
                warn!(
                    "[WARNING] iceoryx2 initialization failed: {}. \
                     Falling back to D-Bus GVariant data plane. \
                     Check /dev/shm permissions and OS shared memory limits.",
                    e
                );
                None
            }
        };

        Ok(Self {
            ingest_engine,
            analyze_engine,
            shm_publisher,
        })
    }

    /// Publish bytes to the iceoryx2 arena if available.
    /// Returns true if published, false if fallback needed.
    fn try_publish(&mut self, data: &[u8], payload_type: PayloadType) -> bool {
        if let Some(ref mut publisher) = self.shm_publisher {
            match publisher.publish(data, payload_type) {
                Ok(()) => true,
                Err(e) => {
                    error!("[ICEORYX2] Publish failed: {}. Falling back to D-Bus.", e);
                    false
                }
            }
        } else {
            false
        }
    }
}

#[dbus_interface(name = "org.nairobi.NairobiAxumRefinery1")]
impl AxumRefineryService {
    /// Ingest a file into a memfd buffer.
    /// Ingestion always returns an FD — the data plane optimization
    /// applies to analytical results, not raw file handles.
    async fn ingest(&mut self, file_path: &str, delimiter: &str, encoding: &str) -> zbus::fdo::Result<OwnedFd> {
        info!("[DBUS] Ingest requested for: {} (delimiter: {}, encoding: {})", file_path, delimiter, encoding);
        self.ingest_engine.ingest(file_path, delimiter, encoding).await.map_err(|e| {
            error!("[DBUS] Ingest failed: {}", e);
            zbus::fdo::Error::Failed(e.to_string())
        })
    }

    /// Analyze a memfd buffer using Polars.
    async fn analyze(
        &mut self,
        handle: OwnedFd,
        query: &str,
    ) -> zbus::fdo::Result<String> {
        info!("[DBUS] Analyze requested for query: {}", query);

        let analytics =
            self.analyze_engine
                .analyze(handle, query)
                .map_err(|e: ImperialError| {
                    error!("[DBUS] Analyze failed: {}", e);
                    zbus::fdo::Error::Failed(e.to_string())
                })?;

        // Serialize to JSON
        let json = serde_json::to_string(&analytics).map_err(|e| {
            zbus::fdo::Error::Failed(format!("JSON serialization failed: {}", e))
        })?;

        // Try iceoryx2 data plane
        if self.try_publish(json.as_bytes(), PayloadType::JsonResult) {
            info!("[ICEORYX2] Analyze result published to arena. {} bytes. Zero kernel copies.", json.len());
            Ok("SHM_READY".to_string())
        } else {
            // Graceful degradation: return JSON over D-Bus
            Ok(json)
        }
    }

    /// Inspect the schema of a memfd buffer.
    async fn inspect_schema(&mut self, handle: OwnedFd) -> zbus::fdo::Result<SchemaInspection> {
        info!("[DBUS] Inspect Schema requested");

        let inspection =
            self.analyze_engine
                .inspect_schema(handle)
                .map_err(|e: ImperialError| {
                    error!("[DBUS] Inspect Schema failed: {}", e);
                    zbus::fdo::Error::Failed(e.to_string())
                })?;

        Ok(inspection)
    }

    /// Clean the data in a memfd buffer and return a new memfd buffer.
    async fn clean_data(
        &mut self,
        handle: OwnedFd,
        strategies: Vec<CleanDataStrategy>,
    ) -> zbus::fdo::Result<OwnedFd> {
        info!(
            "[DBUS] Clean Data requested with {} strategies",
            strategies.len()
        );

        let new_handle =
            self.analyze_engine
                .clean_data(handle, strategies)
                .map_err(|e: ImperialError| {
                    error!("[DBUS] Clean Data failed: {}", e);
                    zbus::fdo::Error::Failed(e.to_string())
                })?;

        Ok(new_handle)
    }

    /// Execute a SQL query on a memfd buffer and return result via arena or D-Bus.
    async fn sql_query(&mut self, handle: OwnedFd, query: &str) -> zbus::fdo::Result<OwnedFd> {
        info!("[DBUS] SQL Query requested: {}", query);

        let new_handle =
            self.analyze_engine
                .sql_query(handle, query)
                .await
                .map_err(|e: ImperialError| {
                    error!("[DBUS] SQL Query failed: {}", e);
                    zbus::fdo::Error::Failed(e.to_string())
                })?;

        Ok(new_handle)
    }

    /// Calculate correlation between two columns and return the results.
    async fn correlation(
        &mut self,
        handle: OwnedFd,
        query: &str,
    ) -> zbus::fdo::Result<String> {
        info!("[DBUS] Correlation requested: {}", query);

        let result =
            self.analyze_engine
                .correlation(handle, query)
                .map_err(|e: ImperialError| {
                    error!("[DBUS] Correlation failed: {}", e);
                    zbus::fdo::Error::Failed(e.to_string())
                })?;

        let json = serde_json::to_string(&result).map_err(|e| {
            zbus::fdo::Error::Failed(format!("JSON serialization failed: {}", e))
        })?;

        if self.try_publish(json.as_bytes(), PayloadType::JsonResult) {
            info!("[ICEORYX2] Correlation result published to arena. {} bytes.", json.len());
            Ok("SHM_READY".to_string())
        } else {
            Ok(json)
        }
    }

    /// Fused crunch + correlate in a single D-Bus call.
    /// Parses CSV once, computes both analytics and correlation.
    async fn crunch_and_correlate(
        &mut self,
        handle: OwnedFd,
        column: &str,
        corr_columns: &str,
    ) -> zbus::fdo::Result<String> {
        info!(
            "[DBUS] Fused CrunchAndCorrelate: column={}, corr={}",
            column, corr_columns
        );

        let result = self
            .analyze_engine
            .crunch_and_correlate(handle, column, corr_columns)
            .map_err(|e: ImperialError| {
                error!("[DBUS] CrunchAndCorrelate failed: {}", e);
                zbus::fdo::Error::Failed(e.to_string())
            })?;

        let json = serde_json::to_string(&result).map_err(|e| {
            zbus::fdo::Error::Failed(format!("JSON serialization failed: {}", e))
        })?;

        if self.try_publish(json.as_bytes(), PayloadType::JsonResult) {
            info!("[ICEORYX2] CrunchAndCorrelate result published to arena. {} bytes. Zero kernel copies.", json.len());
            Ok("SHM_READY".to_string())
        } else {
            Ok(json)
        }
    }

    /// Full pipeline in a single D-Bus call: ingest → crunch → correlate.
    /// Eliminates all intermediate D-Bus round trips.
    async fn ingest_crunch_correlate(
        &mut self,
        file_path: &str,
        delimiter: &str,
        encoding: &str,
        column: &str,
        corr_columns: &str,
    ) -> zbus::fdo::Result<String> {
        info!(
            "[DBUS] Fused IngestCrunchCorrelate: file={} (delimiter: {}, encoding: {}), column={}, corr={}",
            file_path, delimiter, encoding, column, corr_columns
        );

        // 1. Ingest
        let handle = self.ingest_engine.ingest(file_path, delimiter, encoding).await.map_err(|e| {
            error!("[DBUS] IngestCrunchCorrelate ingest failed: {}", e);
            zbus::fdo::Error::Failed(e.to_string())
        })?;

        // 2. Fused crunch + correlate (single CSV parse)
        let result = self
            .analyze_engine
            .crunch_and_correlate(handle, column, corr_columns)
            .map_err(|e: ImperialError| {
                error!("[DBUS] IngestCrunchCorrelate analysis failed: {}", e);
                zbus::fdo::Error::Failed(e.to_string())
            })?;

        let json = serde_json::to_string(&result).map_err(|e| {
            zbus::fdo::Error::Failed(format!("JSON serialization failed: {}", e))
        })?;

        if self.try_publish(json.as_bytes(), PayloadType::JsonResult) {
            info!(
                "[ICEORYX2] IngestCrunchCorrelate result published to arena. {} bytes. Zero kernel copies.",
                json.len()
            );
            Ok("SHM_READY".to_string())
        } else {
            Ok(json)
        }
    }
}
