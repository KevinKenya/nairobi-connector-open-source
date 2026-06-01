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
use tracing::info;
use zbus::zvariant::OwnedFd;
use std::os::unix::io::{AsRawFd, FromRawFd};

enum NodeResult {
    Handle(OwnedFd),
}

/// Duplicate a file descriptor with error checking.
fn duplicate_fd(fd: &OwnedFd) -> Result<OwnedFd, ImperialError> {
    let new_fd = unsafe { libc::dup(fd.as_raw_fd()) };
    if new_fd < 0 {
        return Err(ImperialError::SystemicSeizure(format!(
            "dup() failed: {}",
            std::io::Error::last_os_error()
        )));
    }
    Ok(unsafe { OwnedFd::from_raw_fd(new_fd) })
}

/// Extract a required string parameter from node params, returning an error if missing or empty.
fn require_param<'a>(
    params: &'a serde_json::Value,
    key: &str,
    node_id: u32,
    node_type: &str,
) -> Result<&'a str, ImperialError> {
    params
        .get(key)
        .and_then(|p| p.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            ImperialError::Codec(format!(
                "Node {}: {} requires a non-empty '{}' parameter",
                node_id, node_type, key
            ))
        })
}

/// Resolve the input FD for a node that requires exactly one input edge.
fn resolve_input_fd(
    node_id: u32,
    node_type: &str,
    input_edges: &[u32],
    result_store: &std::collections::HashMap<u32, NodeResult>,
) -> Result<OwnedFd, ImperialError> {
    let input_id = input_edges.first().ok_or_else(|| {
        ImperialError::SystemicSeizure(format!(
            "Node {}: {} requires input",
            node_id, node_type
        ))
    })?;

    match result_store.get(input_id) {
        Some(NodeResult::Handle(fd)) => duplicate_fd(fd),
        _ => Err(ImperialError::SystemicSeizure(format!(
            "Node {}: Missing input handle from node {}",
            node_id, input_id
        ))),
    }
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
                    let dataset_path = require_param(
                        &node.params, "dataset_path", node.node_id, "Ingest"
                    )?;
                    let fd = self
                        .client
                        .ingest(dataset_path, ",", "utf-8")
                        .await?;
                    result_store.insert(node.node_id, NodeResult::Handle(fd));
                    info!("Node {}: Ingest completed", node.node_id);
                }
                NodeType::SqlQuery => {
                    let input_fd = resolve_input_fd(
                        node.node_id, "SqlQuery", &node.input_edges, &result_store
                    )?;
                    let query = require_param(
                        &node.params, "query", node.node_id, "SqlQuery"
                    )?;
                    let result_fd = self.client.sql_query(input_fd, query).await?;
                    result_store.insert(node.node_id, NodeResult::Handle(result_fd));
                    info!("Node {}: SqlQuery completed", node.node_id);
                }
NodeType::AxiomCrunch => {
                    let input_fd = resolve_input_fd(
                        node.node_id, "AxiomCrunch", &node.input_edges, &result_store
                    )?;
                    let column = require_param(
                        &node.params, "column", node.node_id, "AxiomCrunch"
                    )?;

                    let analytics = self.client.analyze(input_fd, column).await?;
                    let json_result = serde_json::to_string(&analytics).map_err(|e| {
                        ImperialError::Codec(format!("Failed to serialize analytics: {}", e))
                    })?;

                    // Write the JSON result into a memfd so downstream nodes can consume it
                    let memfd_name = std::ffi::CString::new(format!("nairobi-axiom-{}", node.node_id))
                        .map_err(|e| ImperialError::Codec(format!("Invalid memfd name: {}", e)))?;
                    let raw_fd = unsafe { libc::memfd_create(memfd_name.as_ptr(), 0) };
                    if raw_fd < 0 {
                        return Err(ImperialError::SystemicSeizure(format!(
                            "memfd_create failed: {}", std::io::Error::last_os_error()
                        )));
                    }
                    let owned_fd = unsafe { OwnedFd::from_raw_fd(raw_fd) };

                    // Write JSON bytes into the memfd
                    let bytes = json_result.as_bytes();
                    let written = unsafe {
                        libc::write(
                            owned_fd.as_raw_fd(),
                            bytes.as_ptr() as *const libc::c_void,
                            bytes.len(),
                        )
                    };
                    if written < 0 {
                        return Err(ImperialError::SystemicSeizure(format!(
                            "Failed to write analytics to memfd: {}",
                            std::io::Error::last_os_error()
                        )));
                    }

                    // Seek back to start so readers can consume from offset 0
                    unsafe { libc::lseek(owned_fd.as_raw_fd(), 0, libc::SEEK_SET) };

                    result_store.insert(node.node_id, NodeResult::Handle(owned_fd));
                    info!("Node {}: AxiomCrunch completed ({} bytes)", node.node_id, bytes.len());
                }
                NodeType::LagosPlot => {
                    let input_raw_fd = if let Some(&input_id) = node.input_edges.first() {
                        match result_store.get(&input_id) {
                            Some(NodeResult::Handle(fd)) => fd.as_raw_fd(),
                            _ => return Err(ImperialError::SystemicSeizure(format!("Node {}: Missing input handle for LagosPlot", node.node_id))),
                        }
                    } else {
                        return Err(ImperialError::SystemicSeizure(format!(
                            "Node {}: LagosPlot requires input",
                            node.node_id
                        )));
                    };

                    let format = node
                        .params
                        .get("format")
                        .and_then(|p| p.as_str())
                        .unwrap_or("sparkline");

                    let width = node
                        .params
                        .get("width")
                        .and_then(|p| p.as_u64())
                        .unwrap_or(1000) as u32;
                    let height = node
                        .params
                        .get("height")
                        .and_then(|p| p.as_u64())
                        .unwrap_or(400) as u32;

                    info!("Node {}: LagosPlot rendering (format: {}, input_fd: {}, dims: {}x{})", node.node_id, format, input_raw_fd, width, height);

                    // Materialize memfd contents to a temp file so the child process can access the data.
                    // std::process::Command::output() does not inherit arbitrary FDs.
                    let temp_path = format!("/tmp/nairobi-plot-{}.dat", node.node_id);
                    let ext = if format.to_lowercase() == "png" { "png" } else { "jpg" };
                    let output_path = format!("/tmp/lagos-output-{}.{}", node.node_id, ext);

                    // Read all bytes from the input FD via mmap
                    let input_file = unsafe { std::fs::File::from_raw_fd(libc::dup(input_raw_fd)) };
                    let metadata = input_file.metadata().map_err(|e| {
                        ImperialError::SystemicSeizure(format!("Failed to stat input FD: {}", e))
                    })?;
                    let file_size = metadata.len() as usize;

                    if file_size > 0 {
                        let mmap = unsafe {
                            memmap2::Mmap::map(&input_file).map_err(|e| {
                                ImperialError::SystemicSeizure(format!("Failed to mmap input FD: {}", e))
                            })?
                        };
                        std::fs::write(&temp_path, &mmap[..]).map_err(|e| {
                            ImperialError::SystemicSeizure(format!("Failed to write temp file: {}", e))
                        })?;
                    } else {
                        std::fs::write(&temp_path, b"").map_err(|e| {
                            ImperialError::SystemicSeizure(format!("Failed to write temp file: {}", e))
                        })?;
                    }

                    let exe_path = std::env::current_exe()
                        .map_err(|e| ImperialError::SystemicSeizure(format!("Failed to get exe path: {}", e)))?;
                    let bin_dir = exe_path.parent()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or("/usr/bin".to_string());

                    let lagos_bin = std::env::var("LAGOS_VISION_DAEMON_BIN")
                        .unwrap_or_else(|_| format!("{}/lagos-vision-daemon", bin_dir));

                    let output = std::process::Command::new(lagos_bin)
                        .arg("--file")
                        .arg(&temp_path)
                        .arg("--format")
                        .arg(format)
                        .arg("--output")
                        .arg(&output_path)
                        .arg("--width")
                        .arg(width.to_string())
                        .arg("--height")
                        .arg(height.to_string())
                        .output()
                        .map_err(|e| ImperialError::SystemicSeizure(format!("Failed to spawn lagos-vision-daemon: {}", e)))?;

                    // Best-effort cleanup of temp file
                    let _ = std::fs::remove_file(&temp_path);

                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(ImperialError::SystemicSeizure(format!("LagosPlot failed: {}", stderr)));
                    }

                    // Verify output file was created
                    if !std::path::Path::new(&output_path).exists() {
                        return Err(ImperialError::SystemicSeizure(format!(
                            "LagosPlot: Output file not created at {}", output_path
                        )));
                    }

                    info!("Node {}: LagosPlot completed, output: {}", node.node_id, output_path);

                    let mut cmd = std::process::Command::new("xdg-open");
                    cmd.arg(&output_path);
                    if let Ok(disp) = std::env::var("DISPLAY") {
                        cmd.env("DISPLAY", disp);
                    }
                    match cmd.spawn() {
                        Ok(_) => info!("Node {}: Launched xdg-open for visualization", node.node_id),
                        Err(e) => info!("Node {}: Failed to launch xdg-open: {}", node.node_id, e),
                    }
                }
            }
        }