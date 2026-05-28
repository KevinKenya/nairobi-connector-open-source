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

use crate::nodes::NairobiNode;
use egui_snarl::Snarl;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;
use zvariant::{EncodingContext as Context, to_bytes, Type};
use byteorder::LE;

#[derive(Debug, Error, Clone)]
pub enum SovereignError {
    #[error("Cyclic dependency detected in node graph")]
    CyclicDependency,

    #[error("GVariant serialization failed: {0}")]
    Serialization(String),

    #[error("IO error: {0}")]
    Io(String),
}

impl From<std::io::Error> for SovereignError {
    fn from(err: std::io::Error) -> Self {
        SovereignError::Io(err.to_string())
    }
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub struct GVariantNode {
    pub node_id: u32,
    pub node_type_and_parameters: String,
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub struct GVariantEdge {
    pub from_node_id: u32,
    pub to_node_id: u32,
}

#[derive(Serialize, Deserialize, Type, Debug, Clone)]
pub struct GVariantDag {
    pub nodes: Vec<GVariantNode>,
    pub edges: Vec<GVariantEdge>,
}

fn get_node_inputs(node: &NairobiNode) -> usize {
    match node {
        NairobiNode::Ingest { .. } => 0,
        NairobiNode::SqlQuery { .. } => 1,
        NairobiNode::AxiomCrunch { .. } => 1,
        NairobiNode::LagosPlot { .. } => 1,
    }
}

pub fn compile_graph(snarl: &Snarl<NairobiNode>) -> Result<Vec<u8>, SovereignError> {
    // 1. Build adjacency list and in-degree map for all active nodes in the snarl
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree: HashMap<usize, usize> = HashMap::new();

    for (node_id, _) in snarl.node_ids() {
        in_degree.insert(node_id.0, 0);
        adj.insert(node_id.0, Vec::new());
    }

    // Populate adjacency list and in-degrees
    for (to_node_id, node) in snarl.node_ids() {
        let num_inputs = get_node_inputs(node);
        for in_idx in 0..num_inputs {
            let in_pin_id = egui_snarl::InPinId {
                node: to_node_id,
                input: in_idx,
            };
            let in_pin = snarl.in_pin(in_pin_id);
            for out_pin_id in in_pin.remotes {
                let from_node_id = out_pin_id.node;
                if adj.contains_key(&from_node_id.0) {
                    adj.get_mut(&from_node_id.0).unwrap().push(to_node_id.0);
                    *in_degree.entry(to_node_id.0).or_insert(0) += 1;
                }
            }
        }
    }

    // 2. Kahn's Topological Sort
    let mut queue = VecDeque::new();
    for (&node_id, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(node_id);
        }
    }

    let mut sorted = Vec::new();
    while let Some(u) = queue.pop_front() {
        sorted.push(u);
        if let Some(neighbors) = adj.get(&u) {
            for &v in neighbors {
                if let Some(deg) = in_degree.get_mut(&v) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(v);
                    }
                }
            }
        }
    }

    if sorted.len() < in_degree.len() {
        return Err(SovereignError::CyclicDependency);
    }

    // 3. Collect edges
    let mut edges = Vec::new();
    let mut seen_edges = HashSet::new();

    for (to_node_id, node) in snarl.node_ids() {
        let num_inputs = get_node_inputs(node);
        for in_idx in 0..num_inputs {
            let in_pin_id = egui_snarl::InPinId {
                node: to_node_id,
                input: in_idx,
            };
            let in_pin = snarl.in_pin(in_pin_id);
            for out_pin_id in in_pin.remotes {
                let from_node_id = out_pin_id.node;
                let edge = (from_node_id.0 as u32, to_node_id.0 as u32);
                if seen_edges.insert(edge) {
                    edges.push(GVariantEdge {
                        from_node_id: edge.0,
                        to_node_id: edge.1,
                    });
                }
            }
        }
    }

    // 4. Build GVariantNode list in topological order
    let mut nodes = Vec::new();
    for node_idx in sorted {
        let node_id = egui_snarl::NodeId(node_idx);
        if let Some(node) = snarl.get_node(node_id) {
            let payload = match node {
                NairobiNode::Ingest { dataset_path } => {
                    serde_json::json!({
                        "type": "Ingest",
                        "dataset_path": dataset_path
                    })
                }
                NairobiNode::SqlQuery { query } => {
                    serde_json::json!({
                        "type": "SqlQuery",
                        "query": query
                    })
                }
                NairobiNode::AxiomCrunch {
                    column,
                    mean,
                    std_dev,
                    kurtosis,
                } => {
                    serde_json::json!({
                        "type": "AxiomCrunch",
                        "column": column,
                        "mean": mean,
                        "std_dev": std_dev,
                        "kurtosis": kurtosis
                    })
                }
                NairobiNode::LagosPlot { format } => {
                    serde_json::json!({
                        "type": "LagosPlot",
                        "format": format
                    })
                }
            };
            let json_str = serde_json::to_string(&payload).map_err(|e| {
                SovereignError::Serialization(format!("JSON serialization failed: {}", e))
            })?;

            nodes.push(GVariantNode {
                node_id: node_idx as u32,
                node_type_and_parameters: json_str,
            });
        }
    }

    // 5. Serialize to GVariant
    let dag = GVariantDag { nodes, edges };
    let ctxt = Context::<LE>::new_gvariant(0);
    let bytes = to_bytes(ctxt, &dag)
        .map_err(|e| SovereignError::Serialization(format!("GVariant serialization error: {}", e)))?;

    Ok(bytes.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::PlotFormat;
    use egui_snarl::Snarl;

    fn make_test_snarl() -> Snarl<NairobiNode> {
        Snarl::new()
    }

    #[test]
    fn test_empty_graph_compilation() {
        let snarl: Snarl<NairobiNode> = make_test_snarl();
        let result = compile_graph(&snarl);
        assert!(result.is_ok());
    }

    #[test]
    fn test_single_node_compilation() {
        let mut snarl: Snarl<NairobiNode> = make_test_snarl();
        snarl.insert_node(
            egui::Pos2::ZERO,
            NairobiNode::Ingest { dataset_path: "/data.csv".to_string() },
        );
        let result = compile_graph(&snarl);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_chain_compilation() {
        let mut snarl: Snarl<NairobiNode> = make_test_snarl();
        let node1 = snarl.insert_node(
            egui::Pos2::ZERO,
            NairobiNode::Ingest { dataset_path: "/data.csv".to_string() },
        );
        let node2 = snarl.insert_node(
            egui::Pos2::new(200.0, 0.0),
            NairobiNode::SqlQuery { query: "SELECT *".to_string() },
        );

        let out_pin_id = egui_snarl::OutPinId { node: node1, output: 0 };
        let in_pin_id = egui_snarl::InPinId { node: node2, input: 0 };
        snarl.connect(out_pin_id, in_pin_id);

        let result = compile_graph(&snarl);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_cyclic_dependency_detection() {
        let mut snarl: Snarl<NairobiNode> = make_test_snarl();
        let node1 = snarl.insert_node(
            egui::Pos2::ZERO,
            NairobiNode::SqlQuery { query: "SELECT 1".to_string() },
        );
        let node2 = snarl.insert_node(
            egui::Pos2::new(200.0, 0.0),
            NairobiNode::AxiomCrunch { column: "value".to_string(), mean: true, std_dev: false, kurtosis: false },
        );

        let out_pin_id1 = egui_snarl::OutPinId { node: node1, output: 0 };
        let in_pin_id2 = egui_snarl::InPinId { node: node2, input: 0 };
        snarl.connect(out_pin_id1, in_pin_id2);

        let out_pin_id2 = egui_snarl::OutPinId { node: node2, output: 0 };
        let in_pin_id1 = egui_snarl::InPinId { node: node1, input: 0 };
        snarl.connect(out_pin_id2, in_pin_id1);

        let result = compile_graph(&snarl);
        assert!(matches!(result, Err(SovereignError::CyclicDependency)));
    }

    #[test]
    fn test_gvariant_node_serialization() {
        let node = GVariantNode {
            node_id: 1,
            node_type_and_parameters: r#"{"type":"Ingest"}"#.to_string(),
        };
        let ctxt = Context::<LE>::new_gvariant(0);
        let bytes = to_bytes(ctxt, &node).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_gvariant_edge_serialization() {
        let edge = GVariantEdge {
            from_node_id: 1,
            to_node_id: 2,
        };
        let ctxt = Context::<LE>::new_gvariant(0);
        let bytes = to_bytes(ctxt, &edge).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_gvariant_dag_serialization() {
        let dag = GVariantDag {
            nodes: vec![GVariantNode { node_id: 1, node_type_and_parameters: "{}".to_string() }],
            edges: vec![GVariantEdge { from_node_id: 1, to_node_id: 2 }],
        };
        let ctxt = Context::<LE>::new_gvariant(0);
        let bytes = to_bytes(ctxt, &dag).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_node_input_output_counts() {
        let mut snarl: Snarl<NairobiNode> = make_test_snarl();

        let ingest = snarl.insert_node(
            egui::Pos2::ZERO,
            NairobiNode::Ingest { dataset_path: "/data".to_string() },
        );
        assert!(snarl.get_node(ingest).is_some());

        let sql = snarl.insert_node(
            egui::Pos2::new(100.0, 0.0),
            NairobiNode::SqlQuery { query: "SELECT".to_string() },
        );
        assert!(snarl.get_node(sql).is_some());

        let axiom = snarl.insert_node(
            egui::Pos2::new(200.0, 0.0),
            NairobiNode::AxiomCrunch { column: "value".to_string(), mean: false, std_dev: false, kurtosis: false },
        );
        assert!(snarl.get_node(axiom).is_some());

        let plot = snarl.insert_node(
            egui::Pos2::new(300.0, 0.0),
            NairobiNode::LagosPlot { format: PlotFormat::Sparkline },
        );
        assert!(snarl.get_node(plot).is_some());

        let node_count = snarl.node_ids().count();
        assert_eq!(node_count, 4);
    }
}