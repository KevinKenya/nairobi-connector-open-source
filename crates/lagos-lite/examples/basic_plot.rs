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

// crates/lagos-lite/examples/basic_plot.rs
// Author: Kevin Chege, Location: Nairobi, Date: 2026-05-27

use lagos_lite::device::HeadlessContext;
use lagos_lite::pipeline::{LagosPipeline, LttbPoint};
use egui_plot::{Line, Plot, PlotPoints};
use std::sync::Arc;

fn main() {
    env_logger::init();
    println!("Starting Lagos Basic Plot Example with NBA data...");

    let workspace_root = std::env::var("CARGO_MANIFEST_DIR")
        .map(|p| std::path::PathBuf::from(p).parent().map(|p| p.parent().map(|p| p.to_path_buf()).unwrap_or_default()).unwrap_or_default())
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());

    let csv_path = workspace_root.join("simulator/PlayerStatisticsExtended.csv");

    let points: Vec<f64> = match std::fs::read_to_string(&csv_path) {
        Ok(content) => {
            content
                .lines()
                .skip(1)
                .filter_map(|line| {
                    line.split(',').nth(19).and_then(|s| s.parse::<f64>().ok())
                })
                .collect()
        }
        Err(e) => {
            eprintln!("Warning: Could not read CSV file {:?}: {}", csv_path, e);
            println!("Using synthetic sine wave data instead.");
            (0..10000).map(|i| (i as f32 * 0.01).sin() as f64).collect()
        }
    };

    let rgba_data = pollster::block_on(async {
        let ctx = Arc::new(HeadlessContext::new().await);
        let mut pipeline = LagosPipeline::new(ctx);

        let input_points: Vec<LttbPoint> = points
            .iter()
            .enumerate()
            .map(|(i, &y)| LttbPoint { x: i as f32, y: y as f32 })
            .collect();

        let width = 1920u32;
        let height = 1080u32;
        let output_count = 2000u32;

        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width as f32, height as f32),
            )),
            ..Default::default()
        };

        pipeline.process_and_render(
            &input_points,
            output_count,
            width,
            height,
            raw_input,
            |ctx, decimated| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let plot_points: PlotPoints = decimated
                        .iter()
                        .map(|p| [p.x as f64, p.y as f64])
                        .collect();
                    let line = Line::new(plot_points);

                    Plot::new("sparkline")
                        .view_aspect(width as f32 / height as f32)
                        .show(ui, |plot_ui| {
                            plot_ui.line(line);
                        });
                });
            },
        ).await
    });

    assert_eq!(rgba_data.len(), (1920 * 1080 * 4) as usize);

    let output_path = workspace_root.join("lagos_basic_plot.png");

    image::RgbaImage::from_raw(1920, 1080, rgba_data)
        .expect("Failed to create image from raw bytes")
        .save(&output_path)
        .expect("Failed to save image");

    println!("Plot saved to: {}", output_path.display());
    println!("Data points rendered: {}", points.len());
}