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

use egui::{Color32, Pos2, Style, Ui};
use egui_snarl::{
    ui::{PinInfo, SnarlViewer},
    InPin, NodeId, OutPin, Snarl,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlotFormat {
    Sparkline,
    Scatter,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NairobiNode {
    Ingest {
        dataset_path: String,
    },
    SqlQuery {
        query: String,
    },
    AxiomCrunch {
        column: String,
        mean: bool,
        std_dev: bool,
        kurtosis: bool,
    },
    LagosPlot {
        format: PlotFormat,
    },
}

pub struct NairobiViewer;

impl SnarlViewer<NairobiNode> for NairobiViewer {
    fn title(&mut self, node: &NairobiNode) -> String {
        match node {
            NairobiNode::Ingest { .. } => "Ingest Node".to_owned(),
            NairobiNode::SqlQuery { .. } => "SQL Query Node".to_owned(),
            NairobiNode::AxiomCrunch { .. } => "Axiom Crunch Node".to_owned(),
            NairobiNode::LagosPlot { .. } => "Lagos Plot Node".to_owned(),
        }
    }

    fn inputs(&mut self, node: &NairobiNode) -> usize {
        match node {
            NairobiNode::Ingest { .. } => 0,
            NairobiNode::SqlQuery { .. } => 1,
            NairobiNode::AxiomCrunch { .. } => 1,
            NairobiNode::LagosPlot { .. } => 1,
        }
    }

    fn outputs(&mut self, node: &NairobiNode) -> usize {
        match node {
            NairobiNode::Ingest { .. } => 1,
            NairobiNode::SqlQuery { .. } => 1,
            NairobiNode::AxiomCrunch { .. } => 1,
            NairobiNode::LagosPlot { .. } => 0,
        }
    }

    fn has_body(&mut self, _node: &NairobiNode) -> bool {
        true
    }

    fn show_body(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<NairobiNode>,
    ) {
        match &mut snarl[node] {
            NairobiNode::Ingest { dataset_path } => {
                ui.label("Dataset Path:");
                ui.text_edit_singleline(dataset_path);
            }
            NairobiNode::SqlQuery { query } => {
                ui.label("Polars SQL Query:");
                ui.text_edit_multiline(query);
            }
            NairobiNode::AxiomCrunch {
                column,
                mean,
                std_dev,
                kurtosis,
            } => {
                ui.label("Column:");
                ui.text_edit_singleline(column);
                ui.label("Statistical Operations:");
                ui.checkbox(mean, "Mean");
                ui.checkbox(std_dev, "Std Dev");
                ui.checkbox(kurtosis, "Kurtosis");
            }
            NairobiNode::LagosPlot { format } => {
                ui.label("Visual Format:");
                ui.radio_value(format, PlotFormat::Sparkline, "Sparkline");
                ui.radio_value(format, PlotFormat::Scatter, "Scatter");
            }
        }
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        _scale: f32,
        _snarl: &mut Snarl<NairobiNode>,
    ) -> PinInfo {
        ui.label(format!("In {}", pin.id.input));
        PinInfo::circle().with_fill(Color32::from_rgb(100, 200, 100))
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        _scale: f32,
        _snarl: &mut Snarl<NairobiNode>,
    ) -> PinInfo {
        ui.label(format!("Out {}", pin.id.output));
        PinInfo::circle().with_fill(Color32::from_rgb(100, 150, 250))
    }

    fn input_color(
        &mut self,
        _pin: &InPin,
        _style: &Style,
        _snarl: &mut Snarl<NairobiNode>,
    ) -> Color32 {
        Color32::from_rgb(100, 200, 100)
    }

    fn output_color(
        &mut self,
        _pin: &OutPin,
        _style: &Style,
        _snarl: &mut Snarl<NairobiNode>,
    ) -> Color32 {
        Color32::from_rgb(100, 150, 250)
    }

    fn graph_menu(
        &mut self,
        pos: Pos2,
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<NairobiNode>,
    ) {
        ui.label("Add Node");
        if ui.button("Ingest Node").clicked() {
            snarl.insert_node(
                pos,
                NairobiNode::Ingest {
                    dataset_path: "".to_owned(),
                },
            );
            ui.close_menu();
        }
        if ui.button("SQL Query Node").clicked() {
            snarl.insert_node(pos, NairobiNode::SqlQuery { query: "".to_owned() });
            ui.close_menu();
        }
        if ui.button("Axiom Crunch Node").clicked() {
            snarl.insert_node(
                pos,
                NairobiNode::AxiomCrunch {
                    column: "".to_owned(),
                    mean: true,
                    std_dev: false,
                    kurtosis: false,
                },
            );
            ui.close_menu();
        }
        if ui.button("Lagos Plot Node").clicked() {
            snarl.insert_node(
                pos,
                NairobiNode::LagosPlot {
                    format: PlotFormat::Sparkline,
                },
            );
            ui.close_menu();
        }
    }

    fn node_menu(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<NairobiNode>,
    ) {
        if ui.button("Remove Node").clicked() {
            snarl.remove_node(node);
            ui.close_menu();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs(node: &NairobiNode) -> usize {
        match node {
            NairobiNode::Ingest { .. } => 0,
            NairobiNode::SqlQuery { .. } => 1,
            NairobiNode::AxiomCrunch { .. } => 1,
            NairobiNode::LagosPlot { .. } => 1,
        }
    }

    fn outputs(node: &NairobiNode) -> usize {
        match node {
            NairobiNode::Ingest { .. } => 1,
            NairobiNode::SqlQuery { .. } => 1,
            NairobiNode::AxiomCrunch { .. } => 1,
            NairobiNode::LagosPlot { .. } => 0,
        }
    }

    #[test]
    fn test_plot_format_variants() {
        assert!(PlotFormat::Sparkline == PlotFormat::Sparkline);
        assert!(PlotFormat::Scatter == PlotFormat::Scatter);
    }

    #[test]
    fn test_nairobi_node_variants() {
        let _ingest = NairobiNode::Ingest { dataset_path: "test".to_string() };
        let _sql_query = NairobiNode::SqlQuery { query: "SELECT *".to_string() };
        let _axiom = NairobiNode::AxiomCrunch { column: "test".to_string(), mean: true, std_dev: false, kurtosis: false };
        let _plot = NairobiNode::LagosPlot { format: PlotFormat::Sparkline };
    }

    #[test]
    fn test_nairobi_node_serialization() {
        let ingest = NairobiNode::Ingest { dataset_path: "/path/to/data.csv".to_string() };
        let json = serde_json::to_string(&ingest).unwrap();
        assert!(json.contains("\"Ingest\""));
        assert!(json.contains("\"dataset_path\":\"/path/to/data.csv\""));

        let sql = NairobiNode::SqlQuery { query: "SELECT * FROM t".to_string() };
        let json = serde_json::to_string(&sql).unwrap();
        assert!(json.contains("\"SqlQuery\""));
        assert!(json.contains("\"query\":\"SELECT * FROM t\""));

        let axiom = NairobiNode::AxiomCrunch { column: "test".to_string(), mean: true, std_dev: true, kurtosis: false };
        let json = serde_json::to_string(&axiom).unwrap();
        assert!(json.contains("\"AxiomCrunch\""));
        assert!(json.contains("\"column\":\"test\""));
        assert!(json.contains("\"mean\":true"));
        assert!(json.contains("\"std_dev\":true"));
        assert!(json.contains("\"kurtosis\":false"));

        let plot = NairobiNode::LagosPlot { format: PlotFormat::Scatter };
        let json = serde_json::to_string(&plot).unwrap();
        assert!(json.contains("\"LagosPlot\""));
        assert!(json.contains("\"Sparkline\"") || json.contains("\"Scatter\""));
    }

    #[test]
    fn test_nairobi_node_deserialization() {
        let ingest: NairobiNode = serde_json::from_str(
            r#"{"Ingest":{"dataset_path":"/data.csv"}}"#
        ).unwrap();
        assert!(matches!(ingest, NairobiNode::Ingest { .. }));

        let sql: NairobiNode = serde_json::from_str(
            r#"{"SqlQuery":{"query":"SELECT 1"}}"#
        ).unwrap();
        assert!(matches!(sql, NairobiNode::SqlQuery { .. }));

        let axiom: NairobiNode = serde_json::from_str(
            r#"{"AxiomCrunch":{"column":"price","mean":true,"std_dev":false,"kurtosis":true}}"#
        ).unwrap();
        assert!(matches!(axiom, NairobiNode::AxiomCrunch { .. }));

        let plot: NairobiNode = serde_json::from_str(
            r#"{"LagosPlot":{"format":"Sparkline"}}"#
        ).unwrap();
        assert!(matches!(plot, NairobiNode::LagosPlot { .. }));
    }

    #[test]
    fn test_node_input_output_counts() {
        let ingest = NairobiNode::Ingest { dataset_path: "".to_string() };
        assert_eq!(inputs(&ingest), 0);
        assert_eq!(outputs(&ingest), 1);

        let sql = NairobiNode::SqlQuery { query: "".to_string() };
        assert_eq!(inputs(&sql), 1);
        assert_eq!(outputs(&sql), 1);

        let axiom = NairobiNode::AxiomCrunch { column: "".to_string(), mean: false, std_dev: false, kurtosis: false };
        assert_eq!(inputs(&axiom), 1);
        assert_eq!(outputs(&axiom), 1);

        let plot = NairobiNode::LagosPlot { format: PlotFormat::Sparkline };
        assert_eq!(inputs(&plot), 1);
        assert_eq!(outputs(&plot), 0);
    }
}
