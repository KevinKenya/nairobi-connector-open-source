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

use crate::dag_parser::NodeType;
use nairobi_protocol::ImperialError;
use tracing::{info, warn};
use zbus::zvariant::OwnedFd;
use std::os::unix::io::{AsRawFd, FromRawFd};

enum NodeResult {
    Handle(OwnedFd),
    Json(String),
}

fn duplicate_fd(fd: &OwnedFd) -> OwnedFd {
    unsafe { OwnedFd::from_raw_fd(libc::dup(fd.as_raw_fd())) }
}

pub struct DagExecutor {
    client: crate::RefineryClient,
}

impl DagExecutor {
    pub async fn new() -> Result<Self, ImperialError> {
        let client = crate::RefineryClient::connect().await?;
        Ok(Self { client })
    }

    pub async fn execute(&mut self, dag_bytes: Vec<u8>) -> Result<String, ImperialError> {
        let parsed_nodes = crate::dag_parser::parse_dag(&dag_bytes).map_err(|e| {
            ImperialError::Codec(format!("Failed to parse DAG: {}", e))
        })?;

        let mut result_store: std::collections::HashMap<u32, NodeResult> = std::collections::HashMap::new();

        for node in parsed_nodes {
            match node.node_type {
                NodeType::Ingest => {
                    let dataset_path = node
                        .params
                        .get("dataset_path")
                        .and_then(|p| p.as_str())
                        .unwrap_or("");
                    let fd = self
                        .client
                        .ingest(dataset_path, ",", "utf-8")
                        .await?;
                    result_store.insert(node.node_id, NodeResult::Handle(fd));
                    info!("Node {}: Ingest completed", node.node_id);
                }
                NodeType::SqlQuery => {
                    let input_fd = if let Some(&input_id) = node.input_edges.first() {
                        match result_store.get(&input_id) {
                            Some(NodeResult::Handle(fd)) => duplicate_fd(fd),
                            _ => return Err(ImperialError::SystemicSeizure(format!("Node {}: Missing input handle", node.node_id))),
                        }
                    } else {
                        return Err(ImperialError::SystemicSeizure(format!(
                            "Node {}: SqlQuery requires input",
                            node.node_id
                        )));
                    };

                    let query = node
                        .params
                        .get("query")
                        .and_then(|p| p.as_str())
                        .unwrap_or("");
                    let result_fd = self.client.sql_query(input_fd, query).await?;
                    result_store.insert(node.node_id, NodeResult::Handle(result_fd));
                    info!("Node {}: SqlQuery completed", node.node_id);
                }
                NodeType::AxiomCrunch => {
                    let input_fd = if let Some(&input_id) = node.input_edges.first() {
                        match result_store.get(&input_id) {
                            Some(NodeResult::Handle(fd)) => duplicate_fd(fd),
                            _ => return Err(ImperialError::SystemicSeizure(format!("Node {}: Missing input handle", node.node_id))),
                        }
                    } else {
                        return Err(ImperialError::SystemicSeizure(format!(
                            "Node {}: AxiomCrunch requires input",
                            node.node_id
                        )));
                    };

                    let column = node
                        .params
                        .get("column")
                        .and_then(|p| p.as_str())
                        .unwrap_or("");

                    if column.is_empty() {
                        return Err(ImperialError::Codec(
                            "AxiomCrunch requires a column parameter".to_string(),
                        ));
                    }

                    let analytics = self.client.analyze(input_fd, column).await?;
                    let json_result = serde_json::to_string(&analytics).map_err(|e| {
                        ImperialError::Codec(format!("Failed to serialize analytics: {}", e))
                    })?;
                    result_store.insert(node.node_id, NodeResult::Json(json_result));
                    info!("Node {}: AxiomCrunch completed", node.node_id);
                }
                NodeType::LagosPlot => {
                    warn!("Node {}: LagosPlot is a placeholder", node.node_id);
                }
            }
        }

        Ok("Execution completed successfully".to_string())
    }
}