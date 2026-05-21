// Copyright The TOON Authors
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

// crates/nairobi-connector/src/toon.rs
// Author: Kevin Chege, Location: Nairobi, Date: 20th May 2026

//! TOON Compression Algorithm — Token-Oriented Object Notation.
//!
//! Converts a raw AT-SPI2 UI tree into a hyper-dense Markdown representation
//! that filters to interactive nodes only, strips metadata, assigns sequential
//! IDs, and produces < 500 tokens output in < 50ms.

use crate::engine::UISnapshot;
use atspi::Role;
use std::collections::HashMap;
use std::time::Instant;

/// Interactive roles that are actionable by AI (kept in TOON output).
/// Uses atspi 0.30 Role variants (Button, CheckMenuItem, RadioMenuItem, etc.)
fn is_interactive(role: Role) -> bool {
    matches!(
        role,
        Role::Button
            | Role::ToggleButton
            | Role::MenuItem
            | Role::CheckMenuItem
            | Role::RadioMenuItem
            | Role::Entry
            | Role::PasswordText
            | Role::SpinButton
            | Role::Slider
            | Role::ComboBox
            | Role::ListItem
            | Role::PageTab
            | Role::PageTabList
            | Role::Link
            | Role::TreeItem
            | Role::CheckBox
            | Role::RadioButton
            | Role::DocumentText
            | Role::Terminal
    )
}

/// Static context roles that provide semantic anchoring but no interaction.
fn is_static_context(role: Role) -> bool {
    matches!(role, Role::Label | Role::Heading | Role::Alert | Role::LevelBar)
}

/// Non-interactive roles stripped from TOON output (but we continue to children).
#[allow(dead_code)]
fn is_noise(role: Role) -> bool {
    matches!(
        role,
        Role::Panel
            | Role::Filler
            | Role::Separator
            | Role::Viewport
            | Role::Unknown
            | Role::ScrollBar
            | Role::StatusBar
            | Role::Header
            | Role::Footer
            | Role::Section
    )
}

/// Generate a TOON representation of a UI snapshot tree.
///
/// Returns (toon_string, node_count, elapsed_ms, id_map).
/// The `id_map` maps each sequential TOON node ID to its (D-Bus destination, object path)
/// pair, enabling precise action dispatch to specific child widgets.
pub fn generate_toon(
    snapshot: &UISnapshot,
    max_depth: u32,
) -> (String, u32, u128, HashMap<u32, (String, String)>) {
    let start = Instant::now();
    let mut output = String::with_capacity(4096);
    let mut next_id: u32 = 1;
    let mut node_count: u32 = 0;
    let mut id_map: HashMap<u32, (String, String)> = HashMap::new();

    serialize_toon(
        snapshot, 0, 0, max_depth, &mut output, &mut next_id, &mut node_count, &mut id_map,
    );

    let elapsed = start.elapsed().as_millis();
    (output, node_count, elapsed, id_map)
}

/// Recursively generate TOON lines for interactive nodes only.
fn serialize_toon(
    node: &UISnapshot,
    depth: u32,
    visible_depth: u32,
    max_depth: u32,
    output: &mut String,
    next_id: &mut u32,
    count: &mut u32,
    id_map: &mut HashMap<u32, (String, String)>,
) {
    if depth > max_depth {
        return;
    }

    let is_interactive = is_interactive(node.role);
    let is_static = is_static_context(node.role);
    let mut next_visible_depth = visible_depth;

    if is_interactive || is_static {
        // Indentation based on visible_depth
        let indent = "  ".repeat(visible_depth as usize);
        output.push_str(&indent);

        if is_interactive {
            let id = *next_id;
            *next_id += 1;
            *count += 1;

            // Cache the D-Bus coordinates for this node ID
            id_map.insert(id, (node.destination.clone(), node.object_path.clone()));

            // [ID: N] Role: "Label" (States)
            output.push_str(&format!("[ID: {}] ", id));
        }

        // Human-readable role name (using atspi 0.30 variant names)
        let role_name = match node.role {
            Role::Button => "Button",
            Role::ToggleButton => "ToggleButton",
            Role::MenuItem => "MenuItem",
            Role::CheckMenuItem => "CheckMenu",
            Role::RadioMenuItem => "RadioMenu",
            Role::Entry => "Entry",
            Role::PasswordText => "PasswordEntry",
            Role::SpinButton => "SpinButton",
            Role::Slider => "Slider",
            Role::ComboBox => "ComboBox",
            Role::ListItem => "ListItem",
            Role::PageTab => "Tab",
            Role::PageTabList => "TabList",
            Role::Link => "Link",
            Role::TreeItem => "TreeItem",
            Role::CheckBox => "CheckBox",
            Role::RadioButton => "RadioButton",
            Role::DocumentText => "TextArea",
            Role::Terminal => "Terminal",
            Role::Label => "Label",
            Role::Heading => "Heading",
            Role::Alert => "Alert",
            Role::LevelBar => "LevelBar",
            _ => "Widget",
        };
        output.push_str(role_name);

        // Label
        let label = node.name.trim();
        if !label.is_empty() {
            output.push_str(": \"");
            // Escape quotes in label
            let escaped = label.replace('"', "'");
            output.push_str(&escaped);
            output.push('"');
        } else {
            output.push_str(": \"\"");
        }

        // Compact state badges
        let mut state_badges = Vec::new();
        use atspi::State;
        if node.states.contains(&State::Focused) { state_badges.push("Focused"); }
        if node.states.contains(&State::Checked) { state_badges.push("Checked"); }
        if node.states.contains(&State::Selected) { state_badges.push("Selected"); }

        if node.states.contains(&State::Enabled) || node.states.contains(&State::Sensitive) {
            state_badges.push("Enabled");
        } else {
            state_badges.push("Disabled");
        }

        if node.states.contains(&State::Expandable) {
            if node.states.contains(&State::Expanded) {
                state_badges.push("Expanded");
            } else {
                state_badges.push("Collapsed");
            }
        }

        if !state_badges.is_empty() {
            output.push_str(" (");
            output.push_str(&state_badges.join(", "));
            output.push(')');
        }

        output.push('\n');
        next_visible_depth += 1;
    }

    // Recurse into children
    // If it was a noise node, we didn't increment visible_depth, so hoisting happens naturally.
    for child in &node.children {
        serialize_toon(child, depth + 1, next_visible_depth, max_depth, output, next_id, count, id_map);
    }
}