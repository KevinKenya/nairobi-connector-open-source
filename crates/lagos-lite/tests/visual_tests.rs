// crates/lagos-lite/tests/visual_tests.rs
// Author: Kevin Chege, Location: Nairobi, Date: 10th May 2026

use lagos_lite::pipeline::{LagosPipeline, LttbPoint};
use lagos_lite::device::HeadlessContext;
use nairobi_protocol::MemoryPipe;
use egui_plot::{Line, Plot, PlotPoints};
use std::sync::Arc;
use std::fs::File;
use std::io::Write;

#[pollster::test]
async fn test_visual_rendering_and_telemetry() {
    let width = 1280;
    let height = 720;
    let output_count = 2000;

    // 1. Generate Mock Data with a recognizable pattern
    // A high-frequency sine wave with a massive anomaly spike in the middle
    let mut points = Vec::new();
    let num_points = 100_000;
    for i in 0..num_points {
        let x = i as f32;
        let mut y = (x * 0.1).sin();
        if i > 49500 && i < 50500 {
            y += 10.0; // The Anomaly
        }
        points.push(LttbPoint { x, y });
    }

    // 2. Setup Headless Context
    let ctx = Arc::new(HeadlessContext::new().await);
    let mut pipeline = LagosPipeline::new(ctx.clone());

    // 3. Render Frame 1: Base View
    let base_input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(width as f32, height as f32),
        )),
        ..Default::default()
    };

    let base_rgba = pipeline.process_and_render(
        &points,
        output_count,
        width,
        height,
        base_input,
        |ctx, decimated| {
            render_sparkline(ctx, decimated, width, height);
        },
    ).await;

    save_png("artifact_frame_base.png", width, height, &base_rgba);
    println!("✅ Rendered artifact_frame_base.png");

    // 4. Render Frame 2: Zoomed View (Simulating Telemetry)
    // We'll simulate a zoom by providing an egui event that changes the plot bounds
    // In egui_plot, we can't easily set bounds from outside without state,
    // but we can simulate a scroll event.
    let zoomed_input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(width as f32, height as f32),
        )),
        events: vec![
            egui::Event::PointerMoved(egui::pos2(width as f32 / 2.0, height as f32 / 2.0)),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Line,
                delta: egui::vec2(0.0, 50.0), // Large Zoom in
                modifiers: Default::default(),
            }
        ],
        ..Default::default()
    };

    let zoomed_rgba = pipeline.process_and_render(
        &points,
        output_count,
        width,
        height,
        zoomed_input,
        |ctx, decimated| {
            render_sparkline(ctx, decimated, width, height);
        },
    ).await;

    save_png("artifact_frame_zoomed.png", width, height, &zoomed_rgba);
    println!("✅ Rendered artifact_frame_zoomed.png");
}

fn render_sparkline(ctx: &egui::Context, decimated: &[LttbPoint], width: u32, height: u32) {
    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(egui::Color32::from_rgb(10, 10, 15)))
        .show(ctx, |ui| {
        let plot_points: PlotPoints = decimated
            .iter()
            .map(|p| [p.x as f64, p.y as f64])
            .collect();
        let line = Line::new(plot_points)
            .color(egui::Color32::from_rgb(0, 255, 150))
            .width(1.5);

        Plot::new("visual_test_plot")
            .view_aspect(width as f32 / height as f32)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    });
}

fn save_png(path: &str, width: u32, height: u32, rgba: &[u8]) {
    image::RgbaImage::from_raw(width, height, rgba.to_vec())
        .expect("Failed to create image")
        .save(path)
        .expect("Failed to save image");
}
