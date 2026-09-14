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

//! Manual, hands-on verification for `NeuralSession::wait_for_save`.
//!
//! These are `#[ignore]`d by default because they require a real desktop:
//! a running AT-SPI2 registry, a display, and — for two of the three tests —
//! a human actually pressing Ctrl+S at the right moment. This is intentional:
//! the unit tests in `src/session.rs` cover the pure logic exhaustively, but
//! "does this actually observe a real save in a real editor" is a claim only
//! a live run can honestly back up.
//!
//! ## Running these
//!
//! 1. Open GNOME Text Editor (or another GtkSourceView-based editor — gedit
//!    works too) with a new, untitled buffer.
//! 2. Run one test at a time, since each expects your attention:
//!    ```sh
//!    cargo test --test verify_save_manual -- --ignored --nocapture --test-threads=1
//!    ```
//! 3. Follow the printed prompt for each test.
//!
//! Each test prints exactly what it found (title before/after, saved: bool)
//! rather than just pass/fail — per project convention, verbatim output over
//! a summary, so a false pass is visible even if the assertion is too loose.

use nairobi_connector::engine::DFSEngine;
use nairobi_connector::NeuralSession;
use std::time::Duration;

const EDITOR_TITLE_HINT: &str = "Text Editor";

/// Find a window matching `EDITOR_TITLE_HINT` and print its TOON map, for the
/// human running the test to read off a node ID by hand. Returns `true` if a
/// window was found and mapped successfully.
async fn find_editor_and_print_map(session: &NeuralSession) -> bool {
    if let Err(e) = session.find_window(EDITOR_TITLE_HINT).await {
        eprintln!(
            "Could not find a window matching '{}': {}. Is GNOME Text Editor open?",
            EDITOR_TITLE_HINT, e
        );
        return false;
    }

    match session.get_ui_map(10).await {
        Ok((toon, node_count, _ms)) => {
            println!("Found {} nodes:\n{}", node_count, toon);
            true
        }
        Err(e) => {
            eprintln!("get_ui_map failed: {}", e);
            false
        }
    }
}

#[tokio::test]
#[ignore = "requires a real desktop session with GNOME Text Editor open"]
async fn wait_for_save_detects_a_real_save() {
    let session = NeuralSession::establish()
        .await
        .expect("failed to connect to AT-SPI2 session bus — is a desktop session running?");

    println!(
        "\n>>> Open GNOME Text Editor with an untitled buffer and type something \
         so the title shows a dirty marker (•)."
    );
    find_editor_and_print_map(&session).await;

    // This test is deliberately left as a documented manual procedure rather
    // than fully automated: driving GNOME Text Editor's specific node layout
    // programmatically is brittle across GTK versions and themes, and a
    // brittle "automated" test that silently no-ops is worse than an honest
    // manual one. See the module doc for the run procedure. What we CAN
    // assert automatically is the timeout path below, which needs no
    // real editor interaction to be trustworthy.
    println!(
        "Manual step: call nairobi_verify_save via the MCP server on the \
         dirty node's Frame/PageTab ancestor, save the file, and confirm the \
         tool returns saved=true with title_before containing '•' or '*' and \
         title_after containing neither."
    );
}

#[tokio::test]
#[ignore = "requires a real desktop session with a window open"]
async fn wait_for_save_times_out_cleanly_without_a_save() {
    let session = NeuralSession::establish()
        .await
        .expect("failed to connect to AT-SPI2 session bus");

    // Find any window at all — we're only checking the timeout path, which
    // is agnostic to what the window is.
    if session.find_window(EDITOR_TITLE_HINT).await.is_err() {
        eprintln!("Skipping: no window matching '{}' found", EDITOR_TITLE_HINT);
        return;
    }
    let (_toon, node_count, _ms) = session
        .get_ui_map(10)
        .await
        .expect("get_ui_map failed on a window we just found");
    assert!(node_count > 0, "expected at least one node in the found window");

    // node 1 is the first interactive node the TOON generator assigned —
    // deliberately not asserting WHAT it is, just using it to exercise the
    // ancestor-walk + timeout path end to end.
    let start = std::time::Instant::now();
    let result = session.wait_for_save(1, 2).await;
    let elapsed = start.elapsed();

    match result {
        Ok((saved, before, after)) => {
            println!(
                "wait_for_save returned saved={} before='{}' after='{}' in {:?}",
                saved, before, after, elapsed
            );
            // We did not save anything, so this must not report success.
            assert!(!saved, "reported saved=true with no save performed");
            // Must respect the requested timeout, not hang or return instantly.
            assert!(
                elapsed >= Duration::from_secs(2) && elapsed < Duration::from_secs(5),
                "timeout was not respected: elapsed={:?}",
                elapsed
            );
        }
        Err(e) => {
            // A NodeNotFound is an acceptable outcome if node 1 wasn't in an
            // editor-shaped ancestor chain — print it so it's a visible,
            // inspectable result rather than a silent pass.
            println!("wait_for_save returned an error (acceptable for a non-editor node): {}", e);
        }
    }
}

#[tokio::test]
#[ignore = "requires a real desktop session"]
async fn wait_for_save_rejects_unknown_node_id() {
    let session = NeuralSession::establish()
        .await
        .expect("failed to connect to AT-SPI2 session bus");

    let result = session.wait_for_save(u32::MAX, 1).await;
    println!("wait_for_save(unknown id) returned: {:?}", result.as_ref().err());
    assert!(result.is_err(), "expected an error for an unpopulated node id");
}

/// Sanity check that `get_parent` degrades gracefully (returns `None`, does
/// not panic or hang) when pointed at the AT-SPI2 registry root itself, which
/// has no parent. This exercises the same DFSEngine method `wait_for_save`
/// relies on to detect the top of the tree.
#[tokio::test]
#[ignore = "requires a running AT-SPI2 registry"]
async fn registry_root_has_no_parent() {
    let session = NeuralSession::establish()
        .await
        .expect("failed to connect to AT-SPI2 session bus");

    let proxy = DFSEngine::timeout_proxy_build(
        session.connection(),
        "org.a11y.atspi.Registry",
        "/org/a11y/atspi/accessible/root",
    )
    .await
    .expect("failed to build proxy for the AT-SPI2 registry root");

    let parent = DFSEngine::get_parent(&proxy).await;
    println!("registry root parent: {:?}", parent);
    assert!(parent.is_none(), "the registry root unexpectedly reported a parent");
}
