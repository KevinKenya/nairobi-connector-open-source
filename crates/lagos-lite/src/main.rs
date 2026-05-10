// crates/lagos-lite/src/main.rs
// Author: Kevin Chege, Location: Nairobi, Date: 10th May 2026

use clap::Parser;
use lagos_lite::{SovereignStream, pipeline::LttbPoint};
use nairobi_protocol::MemoryPipe;
use egui_plot::{Line, Plot, PlotPoints};
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    handle: i32,

    #[arg(short, long, default_value_t = 0)]
    port: u16,

    #[arg(short, long, default_value_t = 1280)]
    width: u32,

    #[arg(short, long, default_value_t = 720)]
    height: u32,

    #[arg(short, long, default_value_t = 2000)]
    output_count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    println!("🚀 Lagos Vision Daemon Igniting...");
    println!("📍 Handle (FD): {}", args.handle);
    println!("📏 Resolution: {}x{}", args.width, args.height);

    // 1. Map the MemoryPipe from the inherited FD
    // SAFETY: We trust the parent process (Nairobi OS) to provide a valid, sealed memfd.
    let mmap = unsafe { MemoryPipe::map_fd(args.handle)? };
    let data_slice: &[LttbPoint] = bytemuck::cast_slice(&mmap);

    // We'll keep the data in an Arc to share with the rendering closures
    let shared_data = Arc::new(data_slice.to_vec());
    let points_count = shared_data.len();
    println!("📊 Points Loaded: {}", points_count);

    // 2. Initialize the Stream
    let stream = SovereignStream::new().await;

    // 3. Start the engine with the High-Density Sparkline UI
    let data_for_get = shared_data.clone();
    stream.start(
        args.port,
        args.width,
        args.height,
        args.output_count,
        move || (*data_for_get).clone(),
        move |ctx, decimated| {
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

                Plot::new("lagos_sparkline")
                    .view_aspect(args.width as f32 / args.height as f32)
                    .allow_drag(true)
                    .allow_zoom(true)
                    .allow_scroll(true)
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    });
            });
        },
    );

    // Keep the main thread alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
}
