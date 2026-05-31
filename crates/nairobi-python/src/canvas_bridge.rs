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

// File: crates/nairobi-python/src/canvas_bridge.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-25

use eframe::egui;
use egui_snarl::Snarl;
use nairobi_canvas::{compile_graph, build_dag_from_config, get_file_picker, NairobiNode, NairobiViewer, NodeConfig, PlotFormat, QueryPreset};
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use tracing::info;

pub struct PythonCanvasApp {
    snarl: Snarl<NairobiNode>,
    viewer: NairobiViewer,
    result: Arc<Mutex<Option<Vec<u8>>>>,
    auto_execute: bool,
}

impl PythonCanvasApp {
    fn new(cc: &eframe::CreationContext<'_>, result: Arc<Mutex<Option<Vec<u8>>>>, auto_execute: bool) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            snarl: Snarl::new(),
            viewer: NairobiViewer,
            result,
            auto_execute,
        }
    }
}

impl eframe::App for PythonCanvasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Nairobi Canvas");
                ui.separator();

                if ui.button("Compile & Close").clicked() {
                    match compile_graph(&self.snarl) {
                        Ok(bytes) => {
                            *self.result.lock().unwrap() = Some(bytes.clone());
                            if self.auto_execute {
                                if let Err(e) = execute_dag_internal(bytes) {
                                    eprintln!("Auto-execution error: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Compilation error: {}", e);
                        }
                    }
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                if ui.button("Cancel").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
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

        // Handle file picker request from node UI
        if let Ok(mut browse_node) = get_file_picker().lock() {
            if let Some(node_id) = *browse_node {
                *browse_node = None;
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("CSV Files", &["csv"])
                    .add_filter("All Files", &["*"])
                    .pick_file()
                {
                    if let Some(node) = self.snarl.get_node_mut(node_id) {
                        if let NairobiNode::Ingest { dataset_path } = node {
                            if let Some(p) = path.to_str() {
                                *dataset_path = p.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
}

#[pyfunction(signature = (preset = None, auto_execute = false))]
pub fn open(_py: Python, preset: Option<String>, auto_execute: Option<bool>) -> PyResult<Option<Vec<u8>>> {
    let result = Arc::new(Mutex::new(None::<Vec<u8>>));
    let should_auto_execute = auto_execute.unwrap_or(false);

    let native_options = eframe::NativeOptions::default();

    let result_for_closure = result.clone();
    let _ = eframe::run_native(
        "Nairobi Canvas",
        native_options,
        Box::new(move |cc| {
            let mut app = Box::new(PythonCanvasApp::new(cc, result_for_closure.clone(), should_auto_execute));

            if let Some(p) = &preset {
                populate_preset(&mut app.snarl, p);
            }

            app
        }),
    );

    let output = result.lock().unwrap().take();
    Ok(output)
}

fn populate_preset(snarl: &mut Snarl<NairobiNode>, preset_name: &str) {
    match preset_name {
        "playerstats_vis" => {
            use egui::Pos2;
            
            let node0 = snarl.insert_node(
                Pos2::new(50.0, 50.0),
                NairobiNode::Ingest {
                    dataset_path: "/home/chege/nairobi-connector-open-source/simulator/PlayerStatisticsExtended.csv".to_string(),
                },
            );
            let node1 = snarl.insert_node(
                Pos2::new(300.0, 50.0),
                NairobiNode::SqlQuery {
                    query: "SELECT points FROM dataset".to_string(),
                    preset: QueryPreset::SingleColumn,
                },
            );
            let node2 = snarl.insert_node(
                Pos2::new(550.0, 50.0),
                NairobiNode::LagosPlot {
                    format: PlotFormat::Png,
                    width: 1200,
                    height: 400,
                },
            );

            let out_pin_id = egui_snarl::OutPinId { node: node0, output: 0 };
            let in_pin_id = egui_snarl::InPinId { node: node1, input: 0 };
            snarl.connect(out_pin_id, in_pin_id);

            let out_pin_id = egui_snarl::OutPinId { node: node1, output: 0 };
            let in_pin_id = egui_snarl::InPinId { node: node2, input: 0 };
            snarl.connect(out_pin_id, in_pin_id);
        }
        _ => {
            eprintln!("Unknown preset: {}", preset_name);
        }
    }
}

fn execute_dag_internal(dag_bytes: Vec<u8>) -> PyResult<()> {
    let rt = shared_runtime();

    rt.block_on(async {
        let connection = zbus::Connection::session().await.map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("D-Bus connection failed: {}", e))
        })?;

        let proxy = zbus::Proxy::new(
            &connection,
            "org.nairobi.NairobiHub1",
            "/org/nairobi/NairobiHub1",
            "org.nairobi.NairobiHub1",
        ).await.map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create Hub proxy: {}", e))
        })?;

        let response: String = proxy
            .call_method("ExecuteDag", &(dag_bytes,))
            .await
            .map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("ExecuteDag failed: {}", e))
            })?
            .body()
            .map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to read response: {}", e))
            })?;

        info!("DAG execution result: {}", response);
        Ok(())
    })
}

pub fn init_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(open, m)?)?;
    m.add_function(wrap_pyfunction!(execute, m)?)?;
    m.add_function(wrap_pyfunction!(build_dag, m)?)?;
    Ok(())
}

use std::sync::OnceLock;

/// Shared Tokio runtime for D-Bus operations — avoids spinning up a new
/// multi-threaded runtime on every `canvas.execute()` call.
fn shared_runtime() -> &'static tokio::runtime::Runtime {
    static RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
    RT.get_or_init(|| {
        tokio::runtime::Runtime::new().expect("Failed to create shared Tokio runtime")
    })
}

#[pyfunction]
pub fn execute(py: Python, dag_bytes: Vec<u8>) -> PyResult<()> {
    let rt = shared_runtime();

    py.allow_threads(|| {
        rt.block_on(async {
            let connection = zbus::Connection::session().await.map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("D-Bus connection failed: {}", e))
            })?;

            let proxy = zbus::Proxy::new(
                &connection,
                "org.nairobi.NairobiHub1",
                "/org/nairobi/NairobiHub1",
                "org.nairobi.NairobiHub1",
            ).await.map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create Hub proxy: {}", e))
            })?;

            let response: String = proxy
                .call_method("ExecuteDag", &(dag_bytes,))
                .await
                .map_err(|e| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!("ExecuteDag failed: {}", e))
                })?
                .body()
                .map_err(|e| {
                    pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to read response: {}", e))
                })?;

            info!("DAG execution result: {}", response);
            Ok::<(), PyErr>(())
        })
    })
}

#[pyfunction]
pub fn build_dag(_py: Python, nodes: &pyo3::types::PyList, edges: &pyo3::types::PyList) -> PyResult<Vec<u8>> {
    let nodes_vec: Vec<(u32, NodeConfig)> = nodes.iter()
        .map(|item| {
            let py_tuple = item.downcast::<pyo3::types::PyTuple>()?;
            let node_id: u32 = py_tuple.get_item(0)?.extract()?;
            let node_type: String = py_tuple.get_item(1)?.extract()?;
            let params_str: String = py_tuple.get_item(2)?.extract()?;
            let params: serde_json::Value = serde_json::from_str(&params_str)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON params: {}", e)))?;
            Ok((node_id, NodeConfig { node_type, params }))
        })
        .collect::<PyResult<Vec<_>>>()?;
    
    let edges_vec: Vec<(u32, u32)> = edges.iter()
        .map(|item| {
            let py_tuple = item.downcast::<pyo3::types::PyTuple>()?;
            let from_id: u32 = py_tuple.get_item(0)?.extract()?;
            let to_id: u32 = py_tuple.get_item(1)?.extract()?;
            Ok((from_id, to_id))
        })
        .collect::<PyResult<Vec<_>>>()?;
    
    let config = nairobi_canvas::DagConfig {
        nodes: nodes_vec,
        edges: edges_vec,
    };
    
    build_dag_from_config(config).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("DAG build failed: {}", e))
    })
}