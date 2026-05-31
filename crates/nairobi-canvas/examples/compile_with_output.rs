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

//! Interactive compilation with file output
//! - Same as canvas_compile_demo but saves compiled GVariant to file
//! - Shows hex dump in console
//! - Demonstrates all node types (Ingest, SqlQuery, AxiomCrunch, LagosPlot)

use eframe::egui;
use egui_snarl::Snarl;
use nairobi_canvas::{compile_graph, NairobiNode, NairobiViewer, PlotFormat, QueryPreset};
use std::path::PathBuf;

const OUTPUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/canvas_compiled_output.bin");

fn get_output_path() -> PathBuf {
    PathBuf::from(OUTPUT_FILE)
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Nairobi Canvas Compiler - File Output Demo",
        native_options,
        Box::new(|cc| Box::new(CanvasCompileOutputApp::new(cc))),
    )
}

struct CanvasCompileOutputApp {
    snarl: Snarl<NairobiNode>,
    viewer: NairobiViewer,
    compile_status: String,
    hex_dump: String,
    file_saved: bool,
}

impl CanvasCompileOutputApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            snarl: Snarl::new(),
            viewer: NairobiViewer,
            compile_status: "Ready - Add nodes and connect them to compile".to_owned(),
            hex_dump: String::new(),
            file_saved: false,
        }
    }
}

impl eframe::App for CanvasCompileOutputApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Nairobi Canvas Compiler - File Output Demo");
                ui.separator();

                if ui.button("Compile Graph").clicked() {
                    match compile_graph(&self.snarl) {
                        Ok(bytes) => {
                            self.compile_status = format!(
                                "Success! Compiled {} bytes of GVariant DAG.",
                                bytes.len()
                            );
                            self.hex_dump = bytes
                                .iter()
                                .map(|b| format!("{:02x}", b))
                                .collect::<Vec<_>>()
                                .join(" ");

                            self.file_saved = false;

                            println!("--- Compiled GVariant DAG {} bytes ---", bytes.len());
                            println!("{}", self.hex_dump);
                            println!("--------------------------------------");

                            let output_path = get_output_path();
                            match std::fs::write(&output_path, &bytes) {
                                Ok(_) => {
                                    self.compile_status.push_str(&format!(" Saved to {}", output_path.display()));
                                    self.file_saved = true;
                                    println!("Saved compiled output to: {}", output_path.display());
                                }
                                Err(e) => {
                                    self.compile_status.push_str(&format!(" Failed to save: {}", e));
                                    eprintln!("Failed to save output file: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            self.compile_status = format!("Error: {}", e);
                            self.hex_dump = String::new();
                            self.file_saved = false;
                        }
                    }
                }

                if ui.button("Clear Graph").clicked() {
                    self.snarl = Snarl::new();
                    self.compile_status = "Cleared".to_owned();
                    self.hex_dump = String::new();
                    self.file_saved = false;
                }

                if ui.button("Pre-populate Demo Graph").clicked() {
                    self.snarl = Snarl::new();

                    let ingest = self.snarl.insert_node(
                        egui::Pos2::new(50.0, 50.0),
                        NairobiNode::Ingest {
                            dataset_path: "/tmp/sample_data.csv".to_string(),
                        },
                    );

                    let sql = self.snarl.insert_node(
                        egui::Pos2::new(250.0, 50.0),
                        NairobiNode::SqlQuery {
                            query: "SELECT * FROM data WHERE value > 100".to_string(),
                            preset: QueryPreset::Custom,
                        },
                    );

                    let axiom = self.snarl.insert_node(
                        egui::Pos2::new(450.0, 50.0),
                        NairobiNode::AxiomCrunch {
                            column: "value".to_string(),
                            mean: true,
                            std_dev: true,
                            kurtosis: false,
                        },
                    );

                    let plot = self.snarl.insert_node(
                        egui::Pos2::new(650.0, 50.0),
                        NairobiNode::LagosPlot {
                            format: PlotFormat::Sparkline,
                            width: 1000,
                            height: 400,
                        },
                    );

                    let out_pin_id = egui_snarl::OutPinId { node: ingest, output: 0 };
                    let in_pin_id = egui_snarl::InPinId { node: sql, input: 0 };
                    self.snarl.connect(out_pin_id, in_pin_id);

                    let out_pin_id = egui_snarl::OutPinId { node: sql, output: 0 };
                    let in_pin_id = egui_snarl::InPinId { node: axiom, input: 0 };
                    self.snarl.connect(out_pin_id, in_pin_id);

                    let out_pin_id = egui_snarl::OutPinId { node: axiom, output: 0 };
                    let in_pin_id = egui_snarl::InPinId { node: plot, input: 0 };
                    self.snarl.connect(out_pin_id, in_pin_id);

                    self.compile_status = "Demo graph loaded - click Compile Graph".to_owned();
                    self.file_saved = false;
                }

                ui.separator();
                ui.label(&self.compile_status);
                if self.file_saved {
                    ui.colored_label(egui::Color32::LIGHT_GREEN, "✓ File saved");
                }
            });

            ui.separator();
            if !self.hex_dump.is_empty() {
                ui.collapsing("Hex Dump", |ui| {
                    ui.monospace(&self.hex_dump);
                });
            }
            ui.label("Tip: Right-click the canvas grid to add nodes. Right-click node header to remove.");
            ui.label("Tip: Use 'Pre-populate Demo Graph' to see a complete pipeline.");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let style = egui_snarl::ui::SnarlStyle::default();
            self.snarl.show(
                &mut self.viewer,
                &style,
                egui::Id::new("snarl_canvas_output"),
                ui,
            );
        });
    }
}