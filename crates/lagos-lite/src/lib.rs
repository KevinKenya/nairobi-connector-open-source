// crates/lagos-lite/src/lib.rs
// Author: Kevin Chege, Location: Nairobi, Date: 10th May 2026

pub mod device;
pub mod pipeline;
pub mod encoder;
pub mod input;
pub mod server;

use std::sync::Arc;
use tokio::sync::{mpsc, Notify};
use crate::device::HeadlessContext;
use crate::pipeline::{LagosPipeline, LttbPoint};
use crate::encoder::compress_rgba_to_jpeg;

pub struct SovereignStream {
    ctx: Arc<HeadlessContext>,
    data_notify: Arc<Notify>,
}

impl SovereignStream {
    pub async fn new() -> Self {
        let ctx = Arc::new(HeadlessContext::new().await);
        Self {
            ctx,
            data_notify: Arc::new(Notify::new()),
        }
    }

    pub fn notify_data_changed(&self) {
        self.data_notify.notify_one();
    }

    pub fn start<F>(
        self,
        port: u16,
        width: u32,
        height: u32,
        output_count: u32,
        get_data: impl Fn() -> Vec<LttbPoint> + Send + 'static,
        render_ui: F,
    ) where
        F: Fn(&egui::Context, &[LttbPoint]) + Send + Sync + 'static,
    {
        let (tx_telemetry, rx_telemetry) = mpsc::channel(100);
        let (tx_frames, rx_frames) = mpsc::channel(1);
        let data_notify = self.data_notify.clone();
        let ctx = self.ctx.clone();
        let render_ui = Arc::new(render_ui);

        // Thread A: Tokio Runtime for WebSocket
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                if let Err(e) = server::run_server(port, rx_frames, tx_telemetry).await {
                    log::error!("Server error: {}", e);
                }
            });
        });

        // Thread B: Dedicated Render Thread
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async {
                let mut pipeline = LagosPipeline::new(ctx);
                let mut rx_telemetry = rx_telemetry;
                let mut events = Vec::new();

                loop {
                    // Wait for event or data change
                    tokio::select! {
                        _ = data_notify.notified() => {},
                        Some(event) = rx_telemetry.recv() => {
                            events.push(event);
                        }
                    }

                    // Drain any remaining events
                    while let Ok(event) = rx_telemetry.try_recv() {
                        events.push(event);
                    }

                    let input_points = get_data();

                    let raw_input = egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width as f32, height as f32),
                        )),
                        events: std::mem::take(&mut events),
                        ..Default::default()
                    };

                    let render_ui_clone = render_ui.clone();
                    let rgba_data = pipeline.process_and_render(
                        &input_points,
                        output_count,
                        width,
                        height,
                        raw_input,
                        move |ctx, points| render_ui_clone(ctx, points),
                    ).await;

                    if let Ok(jpeg_data) = compress_rgba_to_jpeg(width, height, &rgba_data, 80) {
                        let _ = tx_frames.send(jpeg_data).await;
                    }
                }
            });
        });
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

        // Generate 10,000 points of a sine wave
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
        let _port = 0;

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

        // Save to PNG
        image::RgbaImage::from_raw(width, height, rgba_data)
            .expect("Failed to create image from raw bytes")
            .save("test_render.png")
            .expect("Failed to save test_render.png");

        println!("Test render saved to test_render.png");
    }
}
