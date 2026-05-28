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
use nairobi_canvas::{compile_graph, NairobiNode, NairobiViewer};
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};
use zbus::Connection;

struct PythonCanvasApp {
    snarl: Snarl<NairobiNode>,
    viewer: NairobiViewer,
    result: Arc<Mutex<Option<Vec<u8>>>>,
}

impl PythonCanvasApp {
    fn new(cc: &eframe::CreationContext<'_>, result: Arc<Mutex<Option<Vec<u8>>>>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            snarl: Snarl::new(),
            viewer: NairobiViewer,
            result,
        }
    }
}

impl eframe::App for PythonCanvasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Nairobi Visual Compiler");
                ui.separator();

                if ui.button("Compile & Close").clicked() {
                    match compile_graph(&self.snarl) {
                        Ok(bytes) => {
                            *self.result.lock().unwrap() = Some(bytes);
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
    }
}

#[pyfunction]
pub fn open(_py: Python) -> PyResult<Option<Vec<u8>>> {
    let result = Arc::new(Mutex::new(None::<Vec<u8>>));

    let native_options = eframe::NativeOptions::default();

    let result_for_closure = result.clone();
    let _ = eframe::run_native(
        "Nairobi Canvas",
        native_options,
        Box::new(move |cc| {
            Box::new(PythonCanvasApp::new(cc, result_for_closure.clone()))
        }),
    );

    let output = result.lock().unwrap().take();
    Ok(output)
}

pub fn init_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(open, m)?)?;
    m.add_function(wrap_pyfunction!(execute, m)?)?;
    Ok(())
}

#[zbus::dbus_proxy(
    interface = "org.nairobi.NairobiHub1",
    default_service = "org.nairobi.NairobiHub1",
    default_path = "/org/nairobi/NairobiHub1"
)]
pub trait Hub {
    async fn execute_dag(&self, dag_bytes: Vec<u8>) -> zbus::Result<String>;
}

#[pyfunction]
pub fn execute(_py: Python, dag_bytes: Vec<u8>) -> PyResult<Option<String>> {
    _py.allow_threads(move || {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        let result = rt.block_on(async {
            let connection = Connection::session().await.map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("D-Bus connection failed: {}", e))
            })?;
            let proxy = HubProxy::new(&connection).await.map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create Hub proxy: {}", e))
            })?;
            proxy.execute_dag(dag_bytes).await.map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("ExecuteDag failed: {}", e))
            })
        });
        result.map(Some)
    })
}