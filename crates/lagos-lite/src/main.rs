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
use memmap2::Mmap;
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
    fd: i32,

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

    // 1. Memory map the FD directly (Zero-Copy Doctrine)
    let file = unsafe { File::from_raw_fd(args.fd) };
    let mmap = unsafe { Mmap::map(&file)? };

    // 2. Parse CSV data from the memfd into LttbPoints
    let points = parse_csv_to_points(&mmap);
    let points = Arc::new(points);

    eprintln!("[LAGOS_DAEMON] Parsed {} data points from memfd ({} bytes).", points.len(), mmap.len());

    if points.is_empty() {
        eprintln!("[LAGOS_DAEMON] ERROR: No valid data points found in memfd.");
        std::process::exit(1);
    }

    // 3. Initialize Lagos Sovereign Stream
    let stream = lagos_lite::SovereignStream::new().await;

    // Grab the notifier BEFORE start() consumes self
    let notifier = stream.get_notifier();

    // 4. Start the render engine
    let points_clone = points.clone();
    stream.start(
        args.width,
        args.height,
        2000, // Decimation target
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
