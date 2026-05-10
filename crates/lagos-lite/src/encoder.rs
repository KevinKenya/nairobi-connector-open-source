// crates/lagos-lite/src/encoder.rs
// Author: Kevin Chege, Location: Nairobi, Date: 10th May 2026

use jpeg_encoder::{Encoder, ColorType};

pub fn compress_rgba_to_jpeg(width: u32, height: u32, rgba_data: &[u8], quality: u8) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    let encoder = Encoder::new(&mut output, quality);
    encoder.encode(rgba_data, width as u16, height as u16, ColorType::Rgba)
        .map_err(|e| format!("Failed to encode JPEG: {}", e))?;

    Ok(output)
}
