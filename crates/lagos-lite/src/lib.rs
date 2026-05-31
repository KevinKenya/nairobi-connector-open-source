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

// crates/lagos-lite/src/lib.rs
// Author: Kevin Chege, Location: Nairobi, Date: 21st May 2026

pub mod device;
pub mod pipeline;

pub use crate::pipeline::LttbPoint;
use std::sync::Arc;
use crate::device::HeadlessContext;
use crate::pipeline::LagosPipeline;

pub struct SovereignFrame {
    ctx: Arc<HeadlessContext>,
}

impl SovereignFrame {
    pub async fn new() -> Self {
        let ctx = Arc::new(HeadlessContext::new().await);
        Self { ctx }
    }

    pub async fn render_once<F>(
        &self,
        width: u32,
        height: u32,
        output_count: u32,
        input_points: &[LttbPoint],
        render_ui: F,
    ) -> Vec<u8>
    where
        F: FnOnce(&egui::Context, &[LttbPoint]),
    {
        let mut pipeline = LagosPipeline::new(self.ctx.clone());

        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width as f32, height as f32),
            )),
            ..Default::default()
        };

        pipeline
            .process_and_render(input_points, output_count, width, height, raw_input, render_ui)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::device::HeadlessContext;
    use crate::pipeline::{LagosPipeline, LttbPoint};
    use egui_plot::{Line, Plot, PlotPoints};
    use std::sync::Arc;

    #[pollster::test]
    async fn test_render_sparkline() {
        let ctx = Arc::new(HeadlessContext::new().await);
        let mut pipeline = LagosPipeline::new(ctx.clone());

        let input_points: Vec<LttbPoint> = (0..10000)
            .map(|i| {
                let x = i as f32;
                let y = (x * 0.01).sin();
                LttbPoint { x, y }
            })
            .collect();

        let width = 1920;
        let height = 1080;
        let output_count = 2000;

        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width as f32, height as f32),
            )),
            ..Default::default()
        };

        let rgba_data = pipeline.process_and_render(
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
        ).await;

        assert_eq!(rgba_data.len(), (width * height * 4) as usize);

        image::RgbaImage::from_raw(width, height, rgba_data)
            .expect("Failed to create image from raw bytes")
            .save("test_render.png")
            .expect("Failed to save test_render.png");

        println!("Test render saved to test_render.png");
    }
}