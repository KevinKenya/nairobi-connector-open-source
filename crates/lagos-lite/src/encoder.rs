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
