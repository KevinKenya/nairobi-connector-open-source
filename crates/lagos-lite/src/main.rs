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
use image::ImageEncoder;
use memmap2::MmapOptions;
use std::fs::File;
use std::os::unix::io::FromRawFd;
use base64::Engine;
use lagos_lite::pipeline::LttbPoint;
use egui_plot::{Line, Plot, PlotPoints};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File descriptor of the memfd containing CSV data
    #[arg(short, long)]
    fd: Option<i32>,

    /// Render format: sparkline, scatter, or points
    #[arg(short, long, default_value_t = String::from("sparkline"))]
    format: String,

    /// Width of the render target
    #[arg(short, long, default_value_t = 1000)]
    width: u32,

    /// Height of the render target
    #[arg(long, default_value_t = 400)]
    height: u32,
}

fn parse_csv_to_points(data: &[u8]) -> Vec<LttbPoint> {
    let text = match std::str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("[LAGOS] ERROR: memfd data is not valid UTF-8");
            return Vec::new();
        }
    };

    text.lines()
        .skip(1)
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

    let fd = match args.fd {
        Some(f) => f,
        None => {
            eprintln!("[LAGOS_DAEMON] ERROR: --fd is required");
            std::process::exit(1);
        }
    };

    let file = unsafe { File::from_raw_fd(fd) };
    let mmap = unsafe { MmapOptions::new().map(&file) }?;
    let points: Vec<LttbPoint> = parse_csv_to_points(&mmap);

    eprintln!("[LAGOS_DAEMON] Parsed {} data points.", points.len());

    if points.is_empty() {
        eprintln!("[LAGOS_DAEMON] ERROR: No valid data points found.");
        std::process::exit(1);
    }

    let frame = lagos_lite::SovereignFrame::new().await;
    let rgba_data = frame.render_once(
        args.width,
        args.height,
        2000,
        &points,
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

    let output_bytes = match args.format.as_str() {
        "png" | "PNG" => {
            let mut output = Vec::new();
            image::codecs::png::PngEncoder::new(&mut output)
                .write_image(&rgba_data, args.width, args.height, image::ColorType::Rgba8)
                .map_err(|e| {
                    eprintln!("[LAGOS_DAEMON] ERROR: Failed to encode PNG: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, e)
                })?;
            output
        }
        _ => {
            let mut output = Vec::new();
            jpeg_encoder::Encoder::new(&mut output, 80)
                .encode(&rgba_data, args.width as u16, args.height as u16, jpeg_encoder::ColorType::Rgba)
                .map_err(|e| {
                    eprintln!("[LAGOS_DAEMON] ERROR: Failed to encode JPEG: {}", e);
                    std::io::Error::new(std::io::ErrorKind::Other, e)
                })?;
            output
        }
    };

    println!("{}", base64::engine::general_purpose::STANDARD.encode(&output_bytes));

    Ok(())
}