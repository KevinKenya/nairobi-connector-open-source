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

// File: /home/KevinKenya/nairobi-connector-open-source/crates/lagos-lite/src/main.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

use clap::Parser;
use memmap2::MmapOptions;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use std::sync::Arc;
use lagos_lite::pipeline::LttbPoint;
use egui_plot::{Line, Plot, PlotPoints};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File descriptor of the memfd containing CSV data
    #[arg(short, long)]
    fd: Option<i32>,

    /// Path to CSV file for one-shot rendering (mutually exclusive with --fd)
    #[arg(short, long)]
    file: Option<String>,

    /// Render format: sparkline, scatter, or points
    #[arg(short, long, default_value_t = String::from("sparkline"))]
    format: String,

    /// Output path for rendered image (one-shot mode requires this)
    #[arg(short, long)]
    output: Option<String>,

    /// Width of the render target
    #[arg(short, long, default_value_t = 1000)]
    width: u32,

    /// Height of the render target
    #[arg(long, default_value_t = 400)]
    height: u32,
}

/// Parse CSV text from the memfd into f64 values.
/// The memfd contains CSV output from Polars SQL (e.g., "points\n12.5\n8.3\n...")
fn parse_csv_to_points(data: &[u8]) -> Vec<LttbPoint> {
    let text = match std::str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("[LAGOS] ERROR: memfd data is not valid UTF-8");
            return Vec::new();
        }
    };

    text.lines()
        .skip(1) // Skip the CSV header row
        .enumerate()
        .filter_map(|(i, line)| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return None;
            }
            match trimmed.parse::<f64>() {
                Ok(val) => Some(LttbPoint {
                    x: i as f32,
                    y: val as f32,
                }),
                Err(_) => None,
            }
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    // Validate mutually exclusive args
    if args.fd.is_some() && args.file.is_some() {
        eprintln!("[LAGOS_DAEMON] ERROR: --fd and --file are mutually exclusive");
        std::process::exit(1);
    }

    // 1. Get CSV data - either from FD or from file
    let points = match (args.fd, &args.file) {
        (Some(fd), None) => {
            // Memory map the FD directly (Zero-Copy Doctrine)
            let file = unsafe { File::from_raw_fd(fd) };
            let mmap = unsafe { MmapOptions::new().map(&file) }?;
            parse_csv_to_points(&mmap)
        }
        (None, Some(file_path)) => {
            // Read CSV from file
            let file = File::open(file_path).map_err(|e| {
                eprintln!("[LAGOS_DAEMON] ERROR: Failed to open file: {}", e);
                std::io::Error::new(std::io::ErrorKind::Other, e)
            })?;
            let mmap = unsafe { MmapOptions::new().map(&file) }.map_err(|e| {
                eprintln!("[LAGOS_DAEMON] ERROR: Failed to mmap file: {}", e);
                std::io::Error::new(std::io::ErrorKind::Other, e)
            })?;
            parse_csv_to_points(&mmap)
        }
        _ => {
            eprintln!("[LAGOS_DAEMON] ERROR: Either --fd or --file is required");
            std::process::exit(1);
        }
    };

    let points = Arc::new(points);

    eprintln!("[LAGOS_DAEMON] Parsed {} data points.", points.len());

    if points.is_empty() {
        eprintln!("[LAGOS_DAEMON] ERROR: No valid data points found.");
        std::process::exit(1);
    }

    // 2. Initialize Lagos Sovereign Stream
    let stream = lagos_lite::SovereignStream::new().await;

    // 3. One-shot mode: render once and save to file
    if let Some(output_path) = &args.output {
        let rgba_data = stream.render_once(
            args.width,
            args.height,
            2000,
            move || points.to_vec(),
            |ctx, decimated| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let plot_points: PlotPoints = decimated
                        .iter()
                        .map(|p| [p.x as f64, p.y as f64])
                        .collect();
                    let line = Line::new(plot_points);

                    Plot::new("lagos_plot")
                        .show(ui, |plot_ui| {
                            plot_ui.line(line);
                        });
                });
            },
        ).await;

        if rgba_data.is_empty() {
            eprintln!("[LAGOS_DAEMON] ERROR: Render produced no data");
            std::process::exit(1);
        }

        // Encode and save to file based on format
        let output_bytes = match args.format.as_str() {
            "png" | "PNG" => {
                lagos_lite::compress_rgba_to_png(args.width, args.height, &rgba_data)
                    .map_err(|e| {
                        eprintln!("[LAGOS_DAEMON] ERROR: Failed to encode PNG: {}", e);
                        std::io::Error::new(std::io::ErrorKind::Other, e)
                    })?
            }
            _ => {
                lagos_lite::compress_rgba_to_jpeg(args.width, args.height, &rgba_data, 80)
                    .map_err(|e| {
                        eprintln!("[LAGOS_DAEMON] ERROR: Failed to encode JPEG: {}", e);
                        std::io::Error::new(std::io::ErrorKind::Other, e)
                    })?
            }
        };

        std::fs::write(output_path, &output_bytes).map_err(|e| {
            eprintln!("[LAGOS_DAEMON] ERROR: Failed to write output file: {}", e);
            e
        })?;

        eprintln!("[LAGOS_DAEMON] Render saved to {}", output_path);
        return Ok(());
    }

    // 4. Interactive mode (original behavior): Start infinite render loop with WebSocket server
    let notifier = stream.get_notifier();
    let points_clone = points.clone();
    stream.start(
        args.width,
        args.height,
        2000,
        move || {
            points_clone.to_vec()
        },
        move |ctx, decimated| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let plot_points: PlotPoints = decimated
                    .iter()
                    .map(|p| [p.x as f64, p.y as f64])
                    .collect();
                let line = Line::new(plot_points);

                Plot::new("lagos_plot")
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    });
            });
        },
    );

    // 5. Trigger the initial render after a brief setup delay
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    notifier.notify_one();

    // Keep the main thread alive while background threads run
    std::future::pending::<()>().await;

    Ok(())
}
