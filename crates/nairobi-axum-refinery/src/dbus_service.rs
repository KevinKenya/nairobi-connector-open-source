// File: /home/chege/nairobi-connector-open-source/crates/nairobi-axum-refinery/src/dbus_service.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-axum-refinery/src/dbus_service.rs
use crate::analyze::AnalyzeEngine;
use crate::ingest::DiracEngine;
use nairobi_protocol::{
    CleanDataStrategy, CorrelationResult, DistilledAnalytics, FusedAnalyticsResult,
    ImperialError, SchemaInspection,
};
use tracing::{error, info};
use zbus::dbus_interface;
use zbus::zvariant::OwnedFd;

pub struct AxumRefineryService {
    ingest_engine: DiracEngine,
    analyze_engine: AnalyzeEngine,
}

impl AxumRefineryService {
    pub fn new(buffer_size: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let ingest_engine = DiracEngine::new(buffer_size)?;
        let analyze_engine = AnalyzeEngine::new()?;

        Ok(Self {
            ingest_engine,
            analyze_engine,
        })
    }
}

#[dbus_interface(name = "org.nairobi.NairobiAxumRefinery1")]
impl AxumRefineryService {
    /// Ingest a file into a memfd buffer.
    async fn ingest(&mut self, file_path: &str) -> zbus::fdo::Result<OwnedFd> {
        info!("[DBUS] Ingest requested for: {}", file_path);
        self.ingest_engine.ingest(file_path).await.map_err(|e| {
            error!("[DBUS] Ingest failed: {}", e);
            zbus::fdo::Error::Failed(e.to_string())
        })
    }

    /// Analyze a memfd buffer using Polars.
    async fn analyze(
        &mut self,
        handle: OwnedFd,
        query: &str,
    ) -> zbus::fdo::Result<DistilledAnalytics> {
        info!("[DBUS] Analyze requested for query: {}", query);

        let analytics =
            self.analyze_engine
                .analyze(handle, query)
                .map_err(|e: ImperialError| {
                    error!("[DBUS] Analyze failed: {}", e);
                    zbus::fdo::Error::Failed(e.to_string())
                })?;

        Ok(analytics)
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

    /// Execute a SQL query on a memfd buffer and return a new memfd buffer.
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
    ) -> zbus::fdo::Result<CorrelationResult> {
        info!("[DBUS] Correlation requested: {}", query);

        let result =
            self.analyze_engine
                .correlation(handle, query)
                .map_err(|e: ImperialError| {
                    error!("[DBUS] Correlation failed: {}", e);
                    zbus::fdo::Error::Failed(e.to_string())
                })?;

        Ok(result)
    }

    /// Fused crunch + correlate in a single D-Bus call.
    /// Parses CSV once, computes both analytics and correlation.
    async fn crunch_and_correlate(
        &mut self,
        handle: OwnedFd,
        column: &str,
        corr_columns: &str,
    ) -> zbus::fdo::Result<FusedAnalyticsResult> {
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

        Ok(result)
    }

    /// Full pipeline in a single D-Bus call: ingest → crunch → correlate.
    /// Eliminates all intermediate D-Bus round trips.
    async fn ingest_crunch_correlate(
        &mut self,
        file_path: &str,
        column: &str,
        corr_columns: &str,
    ) -> zbus::fdo::Result<FusedAnalyticsResult> {
        info!(
            "[DBUS] Fused IngestCrunchCorrelate: file={}, column={}, corr={}",
            file_path, column, corr_columns
        );

        // 1. Ingest
        let handle = self.ingest_engine.ingest(file_path).await.map_err(|e| {
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

        Ok(result)
    }
}
