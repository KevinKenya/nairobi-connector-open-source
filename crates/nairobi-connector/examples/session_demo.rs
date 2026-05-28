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

//! Demonstrates NeuralSession API with real AT-SPI2 connection.
//! - Connects to AT-SPI2 session bus
//! - Lists available applications on the bus
//! - Demonstrates WindowLock safety mechanism on found windows
//! - Shows TOON format generation (if a window is found)
//! - Outputs results to file

use nairobi_connector::{toon, NeuralSession, UISnapshot, WindowLock};
use std::path::PathBuf;

const OUTPUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/session_demo_output.txt");

fn get_output_path() -> PathBuf {
    PathBuf::from(OUTPUT_FILE)
}

fn create_mock_snapshot() -> UISnapshot {
    use atspi::{Role, State};
    UISnapshot {
        role: Role::Frame,
        name: "Example Window".to_string(),
        actions: vec!["activate".to_string(), "close".to_string()],
        states: vec![State::Enabled, State::Visible],
        children: vec![
            UISnapshot {
                role: Role::PushButton,
                name: "OK Button".to_string(),
                actions: vec!["click".to_string()],
                states: vec![State::Enabled, State::Visible],
                children: vec![],
                depth: 1,
                destination: "org.example.App".to_string(),
                object_path: "/org/example/App/button".to_string(),
            },
            UISnapshot {
                role: Role::Entry,
                name: "Username Field".to_string(),
                actions: vec!["activate".to_string()],
                states: vec![State::Enabled, State::Visible],
                children: vec![],
                depth: 1,
                destination: "org.example.App".to_string(),
                object_path: "/org/example/App/entry".to_string(),
            },
        ],
        depth: 0,
        destination: "org.example.App".to_string(),
        object_path: "/org/example/App/window".to_string(),
    }
}

fn generate_toon_from_snapshot(snapshot: &UISnapshot) -> String {
    let (toon_str, node_count, elapsed, id_map) = toon::generate_toon(snapshot, 10);
    let mut output = String::new();
    output.push_str(&format!("TOON Output ({} nodes, {}ms)\n", node_count, elapsed));
    output.push_str(&format!("---\n{}\n---\n", toon_str));
    output.push_str(&format!("\nID Map ({} entries):\n", id_map.len()));
    for (id, (dest, path)) in id_map {
        output.push_str(&format!("  [ID: {}] dest={}, path={}\n", id, dest, path));
    }
    output
}

fn demonstrate_window_lock() -> String {
    let mut output = String::new();
    output.push_str("\n=== WindowLock Safety Demonstration ===\n");

    let test_cases = vec![
        ("firefox", "Mozilla Firefox"),
        ("gnome-keyring", "Password Prompt"),
        ("my-app", "Settings"),
        ("test-app", "Login Window"),
    ];

    for (app_name, window_title) in test_cases {
        let lock = WindowLock::new(app_name.to_string(), window_title.to_string());
        output.push_str(&format!(
            "Window '{}': app='{}' -> shielded={}, elapsed={:?}\n",
            window_title,
            app_name,
            lock.is_locked(),
            lock.elapsed()
        ));
    }

    output
}

#[tokio::main]
async fn main() {
    println!("=== Nairobi Connector Session Demo ===\n");

    // Try to connect to AT-SPI2 real bus
    let mut output = String::new();
    output.push_str("=== AT-SPI2 Connection Attempt ===\n");

    match NeuralSession::establish().await {
        Ok(session) => {
            println!("✓ Connected to AT-SPI2 session bus successfully!");
            output.push_str("Connected to AT-SPI2 session bus successfully.\n\n");

            // List applications by checking registry children using DFSEngine helper
            let registry_dest = "org.a11y.atspi.Registry";
            let registry_path = "/org/a11y/atspi/accessible/root";

            match nairobi_connector::engine::DFSEngine::timeout_proxy_build(
                session.connection(),
                registry_dest,
                registry_path,
            )
            .await
            {
                Ok(proxy) => {
                    let apps = nairobi_connector::engine::DFSEngine::get_children(&proxy).await;
                    output.push_str(&format!("Found {} accessible applications:\n", apps.len()));
                    println!("\nAccessible Applications ({}):", apps.len());
                    for app in apps.iter().take(10) {
                        let name = &app.name;
                        output.push_str(&format!("  - {}\n", name));
                        println!("  - {}", name);
                    }
                    if apps.len() > 10 {
                        output.push_str(&format!("  ... and {} more\n", apps.len() - 10));
                    }
                }
                Err(e) => {
                    output.push_str(&format!("Failed to create registry proxy: {}\n", e));
                    eprintln!("Failed to create registry proxy: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Could not connect to AT-SPI2 session bus: {}", e);
            output.push_str(&format!("Could not connect to AT-SPI2 session bus: {}\n", e));
            output.push_str("Continuing with mock data demonstration...\n");
        }
    }

    // Demonstrate TOON generation with mock data
    output.push_str("\n=== TOON Generation Demo (Mock Data) ===\n");
    let mock_snapshot = create_mock_snapshot();
    let toon_output = generate_toon_from_snapshot(&mock_snapshot);
    output.push_str(&toon_output);
    println!("\n{}", toon_output);

    // Demonstrate WindowLock safety
    output.push_str(&demonstrate_window_lock());
    println!("{}", demonstrate_window_lock());

    // Write output to file
    let output_path = get_output_path();
    if let Err(e) = std::fs::write(&output_path, &output) {
        eprintln!("Failed to write output file: {}", e);
    } else {
        println!("\n✓ Output saved to: {}", output_path.display());
    }
}