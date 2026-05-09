// File: /home/chege/nairobi-connector-open-source/crates/nairobi-hub/src/client.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-08

// nairobi-open-source-release/crates/nairobi-hub/src/client.rs
//
// v0.1.2 REFIT: The Hub now routes data reads through iceoryx2 shared memory
// when the Refinery signals "SHM_READY". Falls back to parsing JSON from D-Bus
// when iceoryx2 is unavailable.

use crate::shm_subscriber::ShmSubscriber;
use nairobi_protocol::{CleanDataStrategy, CorrelationResult, DistilledAnalytics, FusedAnalyticsResult, ImperialError, SchemaInspection};
use tracing::{info, warn};
use zbus::zvariant::OwnedFd;
use zbus::Connection;

/// D-Bus proxy trait — updated for v0.1.2.
///
/// Methods that previously returned typed GVariant structs now return `String`:
///   - "SHM_READY" → data is in the iceoryx2 arena
///   - JSON string → fallback payload (no iceoryx2)
///
/// Methods that return raw FDs (ingest, clean_data, sql_query) are unchanged.
#[zbus::dbus_proxy(
    interface = "org.nairobi.NairobiAxumRefinery1",
    default_service = "org.nairobi.NairobiAxumRefinery1",
    default_path = "/org/nairobi/NairobiAxumRefinery1"
)]
pub trait AxumRefinery {
    async fn ingest(&self, file_path: &str) -> zbus::Result<OwnedFd>;
    async fn analyze(&self, handle: OwnedFd, query: &str) -> zbus::Result<String>;
    async fn inspect_schema(&self, handle: OwnedFd) -> zbus::Result<SchemaInspection>;
    async fn clean_data(
        &self,
        handle: OwnedFd,
        strategies: Vec<CleanDataStrategy>,
    ) -> zbus::Result<OwnedFd>;
    async fn sql_query(&self, handle: OwnedFd, query: &str) -> zbus::Result<OwnedFd>;
    async fn correlation(
        &self,
        handle: OwnedFd,
        query: &str,
    ) -> zbus::Result<String>;
    async fn crunch_and_correlate(
        &self,
        handle: OwnedFd,
        column: &str,
        corr_columns: &str,
    ) -> zbus::Result<String>;
    async fn ingest_crunch_correlate(
        &self,
        file_path: &str,
        column: &str,
        corr_columns: &str,
    ) -> zbus::Result<String>;
}

pub struct RefineryClient {
    proxy: AxumRefineryProxy<'static>,
    /// iceoryx2 shared memory subscriber (Data Plane).
    /// `None` if iceoryx2 initialization failed — reads fall back to JSON over D-Bus.
    shm_subscriber: Option<ShmSubscriber>,
}

impl RefineryClient {
    /// Connects to the D-Bus session and creates the proxy.
    /// Also attempts to initialize the iceoryx2 data plane subscriber.
    pub async fn connect() -> Result<Self, ImperialError> {
        let connection = Connection::session().await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline (Session bus error): {}",
                e
            ))
        })?;

        let proxy = AxumRefineryProxy::new(&connection).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!("Heavy Iron Engine is offline: {}", e))
        })?;

        // Attempt iceoryx2 Data Plane initialization
        let shm_subscriber = match ShmSubscriber::new() {
            Ok(sub) => {
                info!("[ICEORYX2] Hub subscriber initialized. Data plane active.");
                Some(sub)
            }
            Err(e) => {
                warn!(
                    "[WARNING] iceoryx2 subscriber initialization failed: {}. \
                     Reading data from D-Bus GVariant fallback.",
                    e
                );
                None
            }
        };

        Ok(Self {
            proxy,
            shm_subscriber,
        })
    }

    /// Route a D-Bus string response through the SHM data plane.
    /// If signal is "SHM_READY", read from iceoryx2 arena.
    /// Otherwise, treat the signal as the JSON payload itself (fallback).
    fn resolve_shm_or_json(&self, signal: &str) -> Result<String, ImperialError> {
        if signal == "SHM_READY" {
            if let Some(ref subscriber) = self.shm_subscriber {
                let bytes = subscriber
                    .receive_latest()?
                    .ok_or_else(|| {
                        ImperialError::SystemicSeizure(
                            "SHM_READY received but no data in arena".into(),
                        )
                    })?;
                String::from_utf8(bytes).map_err(|e| {
                    ImperialError::Codec(format!("Arena payload is not valid UTF-8: {}", e))
                })
            } else {
                Err(ImperialError::SystemicSeizure(
                    "SHM_READY received but no iceoryx2 subscriber available".into(),
                ))
            }
        } else {
            // Fallback: the signal IS the JSON payload
            Ok(signal.to_string())
        }
    }

    /// Ingests a file into the backend.
    pub async fn ingest(&self, file_path: &str) -> Result<OwnedFd, ImperialError> {
        self.proxy.ingest(file_path).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during ingest: {}",
                e
            ))
        })
    }

    /// Analyzes a memfd buffer using the backend.
    pub async fn analyze(
        &self,
        fd: OwnedFd,
        query: &str,
    ) -> Result<DistilledAnalytics, ImperialError> {
        let signal = self.proxy.analyze(fd, query).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during analyze: {}",
                e
            ))
        })?;

        let json = self.resolve_shm_or_json(&signal)?;
        serde_json::from_str(&json).map_err(|e| {
            ImperialError::Codec(format!("Failed to deserialize DistilledAnalytics: {}", e))
        })
    }

    /// Orchestrates the data strike.
    pub async fn distill(&self, file_path: &str, query: &str) -> Result<String, ImperialError> {
        // 1. Call ingest() to get the FD.
        let fd = self.proxy.ingest(file_path).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during ingest: {}",
                e
            ))
        })?;

        // 2. Call analyze() with the FD to get the DistilledAnalytics.
        let signal = self.proxy.analyze(fd, query).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during analyze: {}",
                e
            ))
        })?;

        let json = self.resolve_shm_or_json(&signal)?;
        let analytics: DistilledAnalytics = serde_json::from_str(&json).map_err(|e| {
            ImperialError::Codec(format!("Failed to deserialize DistilledAnalytics: {}", e))
        })?;

        // 3. Pass the analytics to the SemanticDecoder.
        let report = crate::decoder::generate_report(&analytics);

        // 4. Return the Markdown string.
        Ok(report)
    }

    /// Inspects the schema using the backend.
    pub async fn inspect_schema(&self, fd: OwnedFd) -> Result<SchemaInspection, ImperialError> {
        self.proxy.inspect_schema(fd).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during inspect_schema: {}",
                e
            ))
        })
    }

    /// Cleans the data using the backend.
    pub async fn clean_data(
        &self,
        fd: OwnedFd,
        strategies: Vec<CleanDataStrategy>,
    ) -> Result<OwnedFd, ImperialError> {
        self.proxy.clean_data(fd, strategies).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during clean_data: {}",
                e
            ))
        })
    }

    /// Executes a SQL query using the backend.
    pub async fn sql_query(&self, fd: OwnedFd, query: &str) -> Result<OwnedFd, ImperialError> {
        self.proxy.sql_query(fd, query).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during sql_query: {}",
                e
            ))
        })
    }

    /// Performs analysis directly on an existing FD.
    pub async fn distill_direct(&self, fd: OwnedFd, query: &str) -> Result<String, ImperialError> {
        let signal = self.proxy.analyze(fd, query).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during analyze: {}",
                e
            ))
        })?;

        let json = self.resolve_shm_or_json(&signal)?;
        let analytics: DistilledAnalytics = serde_json::from_str(&json).map_err(|e| {
            ImperialError::Codec(format!("Failed to deserialize DistilledAnalytics: {}", e))
        })?;

        let report = crate::decoder::generate_report(&analytics);
        Ok(report)
    }

    /// Calculates correlation between columns.
    pub async fn correlation(
        &self,
        fd: OwnedFd,
        query: &str,
    ) -> Result<CorrelationResult, ImperialError> {
        let signal = self.proxy.correlation(fd, query).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during correlation: {}",
                e
            ))
        })?;

        let json = self.resolve_shm_or_json(&signal)?;
        serde_json::from_str(&json).map_err(|e| {
            ImperialError::Codec(format!("Failed to deserialize CorrelationResult: {}", e))
        })
    }

    /// Fused crunch + correlate in a single D-Bus round trip.
    pub async fn crunch_and_correlate(
        &self,
        fd: OwnedFd,
        column: &str,
        corr_columns: &str,
    ) -> Result<FusedAnalyticsResult, ImperialError> {
        let signal = self
            .proxy
            .crunch_and_correlate(fd, column, corr_columns)
            .await
            .map_err(|e| {
                ImperialError::SystemicSeizure(format!(
                    "Heavy Iron Engine is offline during crunch_and_correlate: {}",
                    e
                ))
            })?;

        let json = self.resolve_shm_or_json(&signal)?;
        serde_json::from_str(&json).map_err(|e| {
            ImperialError::Codec(format!("Failed to deserialize FusedAnalyticsResult: {}", e))
        })
    }

    /// Full pipeline in a single D-Bus round trip: ingest → crunch → correlate.
    pub async fn ingest_crunch_correlate(
        &self,
        file_path: &str,
        column: &str,
        corr_columns: &str,
    ) -> Result<FusedAnalyticsResult, ImperialError> {
        let signal = self
            .proxy
            .ingest_crunch_correlate(file_path, column, corr_columns)
            .await
            .map_err(|e| {
                ImperialError::SystemicSeizure(format!(
                    "Heavy Iron Engine is offline during ingest_crunch_correlate: {}",
                    e
                ))
            })?;

        let json = self.resolve_shm_or_json(&signal)?;
        serde_json::from_str(&json).map_err(|e| {
            ImperialError::Codec(format!("Failed to deserialize FusedAnalyticsResult: {}", e))
        })
    }
}
