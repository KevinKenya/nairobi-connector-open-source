// crates/lagos-lite/src/input.rs
// Author: Kevin Chege, Location: Nairobi, Date: 10th May 2026

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
