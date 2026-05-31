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

use eframe::egui;
use egui_snarl::Snarl;
use nairobi_canvas::{compile_graph, get_file_picker, NairobiNode, NairobiViewer};

fn main() -> eframe::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let _guard = rt.enter();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Nairobi Canvas & compiler Demo",
        native_options,
        Box::new(|cc| Box::new(CanvasDemoApp::new(cc))),
    )
}

struct CanvasDemoApp {
    snarl: Snarl<NairobiNode>,
    viewer: NairobiViewer,
    compile_status: String,
    hex_dump: String,
}

impl CanvasDemoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            snarl: Snarl::new(),
            viewer: NairobiViewer,
            compile_status: "Ready".to_owned(),
            hex_dump: String::new(),
        }
    }
}

impl eframe::App for CanvasDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Nairobi Visual Compiler");
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
                            
                            // Also print to stdout as requested by verification example task
                            println!("--- Compiled GVariant DAG Hex Dump ---");
                            println!("{}", self.hex_dump);
                            println!("--------------------------------------");
                        }
                        Err(e) => {
                            self.compile_status = format!("Error: {}", e);
                            self.hex_dump = String::new();
                        }
                    }
                }

                if ui.button("Clear Graph").clicked() {
                    self.snarl = Snarl::new();
                    self.compile_status = "Cleared".to_owned();
                    self.hex_dump = String::new();
                }

                ui.separator();
                ui.label(format!("Status: {}", self.compile_status));
            });

            if !self.hex_dump.is_empty() {
                ui.separator();
                ui.collapsing("Hex Dump", |ui| {
                    ui.monospace(&self.hex_dump);
                });
            }
            ui.label("Tip: Right-click the canvas grid area to add new nodes, or right-click a node header to remove it.");
        });

egui::CentralPanel::default().show(ctx, |ui| {
             let style = egui_snarl::ui::SnarlStyle::default();
             self.snarl.show(
                 &mut self.viewer,
                 &style,
                 egui::Id::new("snarl_canvas"),
                 ui,
             );
         });

         // Handle file picker request from Ingest node
         if let Ok(mut browse_node) = get_file_picker().lock() {
             if let Some(node_id) = *browse_node {
                 *browse_node = None;
                 if let Some(path) = rfd::FileDialog::new()
                     .add_filter("CSV Files", &["csv"])
                     .add_filter("All Files", &["*"])
                     .pick_file()
                 {
                     if let Some(NairobiNode::Ingest { dataset_path }) = self.snarl.get_node_mut(node_id) {
                         if let Some(p) = path.to_str() {
                             *dataset_path = p.to_string();
                         }
                     }
                 }
             }
         }
     }
 }
