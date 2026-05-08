// File: /home/chege/nairobi-connector-open-source/crates/nairobi-hub/src/client.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-hub/src/client.rs
use nairobi_protocol::{CleanDataStrategy, DistilledAnalytics, FusedAnalyticsResult, ImperialError, SchemaInspection};
use zbus::zvariant::OwnedFd;
use zbus::Connection;

#[zbus::dbus_proxy(
    interface = "org.nairobi.NairobiAxumRefinery1",
    default_service = "org.nairobi.NairobiAxumRefinery1",
    default_path = "/org/nairobi/NairobiAxumRefinery1"
)]
pub trait AxumRefinery {
    async fn ingest(&self, file_path: &str) -> zbus::Result<OwnedFd>;
    async fn analyze(&self, handle: OwnedFd, query: &str) -> zbus::Result<DistilledAnalytics>;
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
    ) -> zbus::Result<nairobi_protocol::CorrelationResult>;
    async fn crunch_and_correlate(
        &self,
        handle: OwnedFd,
        column: &str,
        corr_columns: &str,
    ) -> zbus::Result<FusedAnalyticsResult>;
    async fn ingest_crunch_correlate(
        &self,
        file_path: &str,
        column: &str,
        corr_columns: &str,
    ) -> zbus::Result<FusedAnalyticsResult>;
}

pub struct RefineryClient {
    proxy: AxumRefineryProxy<'static>,
}

impl RefineryClient {
    /// Connects to the D-Bus session and creates the proxy.
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

        Ok(Self { proxy })
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
        self.proxy.analyze(fd, query).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during analyze: {}",
                e
            ))
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
        let analytics = self.proxy.analyze(fd, query).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during analyze: {}",
                e
            ))
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
        let analytics = self.proxy.analyze(fd, query).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during analyze: {}",
                e
            ))
        })?;

        let report = crate::decoder::generate_report(&analytics);
        Ok(report)
    }

    /// Calculates correlation between columns.
    pub async fn correlation(
        &self,
        fd: OwnedFd,
        query: &str,
    ) -> Result<nairobi_protocol::CorrelationResult, ImperialError> {
        self.proxy.correlation(fd, query).await.map_err(|e| {
            ImperialError::SystemicSeizure(format!(
                "Heavy Iron Engine is offline during correlation: {}",
                e
            ))
        })
    }

    /// Fused crunch + correlate in a single D-Bus round trip.
    pub async fn crunch_and_correlate(
        &self,
        fd: OwnedFd,
        column: &str,
        corr_columns: &str,
    ) -> Result<FusedAnalyticsResult, ImperialError> {
        self.proxy
            .crunch_and_correlate(fd, column, corr_columns)
            .await
            .map_err(|e| {
                ImperialError::SystemicSeizure(format!(
                    "Heavy Iron Engine is offline during crunch_and_correlate: {}",
                    e
                ))
            })
    }

    /// Full pipeline in a single D-Bus round trip: ingest → crunch → correlate.
    pub async fn ingest_crunch_correlate(
        &self,
        file_path: &str,
        column: &str,
        corr_columns: &str,
    ) -> Result<FusedAnalyticsResult, ImperialError> {
        self.proxy
            .ingest_crunch_correlate(file_path, column, corr_columns)
            .await
            .map_err(|e| {
                ImperialError::SystemicSeizure(format!(
                    "Heavy Iron Engine is offline during ingest_crunch_correlate: {}",
                    e
                ))
            })
    }
}
