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

// crates/lagos-lite/src/input.rs
// Author: Kevin Chege, Location: Nairobi, Date: 21st May 2026

use serde::Deserialize;
use egui::{Event, Pos2, Vec2, PointerButton};

#[derive(Debug, Deserialize)]
#[serde(tag = "t")]
pub enum TelemetryEvent {
    #[serde(rename = "d")] // Drag/Move
    Move { x: f32, y: f32, pressed: bool },
    #[serde(rename = "z")] // Zoom/Scroll
    Zoom { dy: f32 },
    #[serde(rename = "c")] // Click
    Click { x: f32, y: f32, pressed: bool, button: u8 },
}

pub fn map_telemetry_to_egui(event: TelemetryEvent) -> Vec<Event> {
    match event {
        TelemetryEvent::Move { x, y, pressed: _ } => {
            vec![Event::PointerMoved(Pos2::new(x, y))]
        }
        TelemetryEvent::Zoom { dy } => {
            vec![Event::MouseWheel {
                unit: egui::MouseWheelUnit::Line,
                delta: Vec2::new(0.0, dy),
                modifiers: Default::default(),
            }]
        }
        TelemetryEvent::Click { x, y, pressed, button } => {
            let egui_button = match button {
                0 => PointerButton::Primary,
                1 => PointerButton::Middle,
                2 => PointerButton::Secondary,
                _ => PointerButton::Primary,
            };
            vec![
                Event::PointerMoved(Pos2::new(x, y)),
                Event::PointerButton {
                    pos: Pos2::new(x, y),
                    button: egui_button,
                    pressed,
                    modifiers: Default::default(),
                }
            ]
        }
    }
}
