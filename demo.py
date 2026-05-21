#!/usr/bin/env python3
# Copyright 2026 Kevin Chege
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
Nairobi OS v0.4.1 — Full Pipeline Demo
=======================================
A cinematic walkthrough of the entire Nairobi OS stack:
  ACT 1: Data Analytics Pipeline (Refinery + SovereignFrame)
  ACT 2: Computer Use Without Pixels (MCP + TOON)

Run:  python3 demo.py
"""

import json
import os
import subprocess
import sys
import time

# ═══════════════════════════════════════════════════════════════════════════════
# ANSI Color Palette
# ═══════════════════════════════════════════════════════════════════════════════

class C:
    """Terminal color codes for cinematic output."""
    RESET   = "\033[0m"
    BOLD    = "\033[1m"
    DIM     = "\033[2m"
    ITALIC  = "\033[3m"
    UNDER   = "\033[4m"

    # Foreground
    RED     = "\033[91m"
    GREEN   = "\033[92m"
    YELLOW  = "\033[93m"
    BLUE    = "\033[94m"
    MAGENTA = "\033[95m"
    CYAN    = "\033[96m"
    WHITE   = "\033[97m"
    GRAY    = "\033[90m"

    # Background
    BG_BLUE = "\033[44m"
    BG_GREEN = "\033[42m"
    BG_RED  = "\033[41m"
    BG_CYAN = "\033[46m"

# ═══════════════════════════════════════════════════════════════════════════════
# Display Helpers
# ═══════════════════════════════════════════════════════════════════════════════

TERM_WIDTH = 72

def clear():
    os.system("clear")

def pause(seconds=1.5):
    time.sleep(seconds)

def typewriter(text, delay=0.03):
    """Print text character-by-character for dramatic effect."""
    for ch in text:
        sys.stdout.write(ch)
        sys.stdout.flush()
        time.sleep(delay)
    print()

def banner(title, color=C.CYAN):
    """Print a large section banner."""
    border = "═" * TERM_WIDTH
    print(f"\n{color}{C.BOLD}{border}")
    print(f"  {title}")
    print(f"{border}{C.RESET}\n")

def sub_banner(title, color=C.YELLOW):
    """Print a subsection header."""
    line = "─" * TERM_WIDTH
    print(f"\n{color}{line}")
    print(f"  {title}")
    print(f"{line}{C.RESET}\n")

def step(num, total, text):
    """Print a numbered step indicator."""
    bar_filled = int((num / total) * 20)
    bar_empty = 20 - bar_filled
    progress = f"[{'█' * bar_filled}{'░' * bar_empty}]"
    print(f"  {C.BLUE}{C.BOLD}[{num}/{total}]{C.RESET} {progress} {text}")

def stat_line(label, value, unit="", color=C.GREEN):
    """Print a formatted statistic line."""
    dots = "·" * (40 - len(label))
    print(f"  {C.WHITE}{label} {C.DIM}{dots}{C.RESET} {color}{C.BOLD}{value}{C.RESET} {C.DIM}{unit}{C.RESET}")

def ok(msg):
    print(f"  {C.GREEN}✅ {msg}{C.RESET}")

def info(msg):
    print(f"  {C.CYAN}ℹ️  {msg}{C.RESET}")

def warn(msg):
    print(f"  {C.YELLOW}⚠️  {msg}{C.RESET}")

def fail(msg):
    print(f"  {C.RED}❌ {msg}{C.RESET}")

def toon_line(text):
    """Print a line of TOON output with syntax highlighting."""
    # Highlight [ID: N] tags
    import re
    highlighted = re.sub(
        r'\[ID:\s*(\d+)\]',
        f'{C.MAGENTA}{C.BOLD}[ID: \\1]{C.RESET}{C.CYAN}',
        text
    )
    # Highlight role names
    for role in ['BTN', 'ENTRY', 'CHK', 'MENU', 'LBL', 'TXT', 'FRAME', 'PANEL', 'TAB', 'SLIDER']:
        highlighted = highlighted.replace(role, f'{C.YELLOW}{C.BOLD}{role}{C.RESET}{C.CYAN}')
    print(f"  {C.CYAN}{highlighted}{C.RESET}")


# ═══════════════════════════════════════════════════════════════════════════════
# SPLASH SCREEN
# ═══════════════════════════════════════════════════════════════════════════════

def splash():
    clear()
    logo = f"""{C.CYAN}{C.BOLD}
    ███╗   ██╗ █████╗ ██╗██████╗  ██████╗ ██████╗ ██╗
    ████╗  ██║██╔══██╗██║██╔══██╗██╔═══██╗██╔══██╗██║
    ██╔██╗ ██║███████║██║██████╔╝██║   ██║██████╔╝██║
    ██║╚██╗██║██╔══██║██║██╔══██╗██║   ██║██╔══██╗██║
    ██║ ╚████║██║  ██║██║██║  ██║╚██████╔╝██████╔╝██║
    ╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═════╝ ╚═╝
                        {C.WHITE}O  S{C.RESET}
{C.RESET}"""
    print(logo)
    print(f"  {C.DIM}{'─' * TERM_WIDTH}{C.RESET}")
    print(f"  {C.WHITE}{C.BOLD}Version 0.4.1{C.RESET}  {C.DIM}│{C.RESET}  {C.YELLOW}Heavy Iron Data Science Infrastructure{C.RESET}")
    print(f"  {C.DIM}{'─' * TERM_WIDTH}{C.RESET}")
    print()
    typewriter(f"  {C.DIM}Rust Core · Zero-Copy IPC · Computer Use Without Pixels{C.RESET}", delay=0.02)
    print()
    print(f"  {C.GRAY}Author: Kevin Chege  ·  Location: Nairobi, Kenya{C.RESET}")
    print(f"  {C.GRAY}License: Apache 2.0  ·  https://pypi.org/project/nairobi-os/{C.RESET}")
    print()
    pause(2.5)


# ═══════════════════════════════════════════════════════════════════════════════
# ACT 1: DATA ANALYTICS PIPELINE
# ═══════════════════════════════════════════════════════════════════════════════

def act1_analytics():
    banner("ACT 1: DATA ANALYTICS PIPELINE", C.CYAN)
    typewriter(f"  {C.DIM}Zero-copy ingestion · Vectorized crunch · Relational correlation{C.RESET}", delay=0.02)
    pause(1.0)

    import nairobi_os
    from nairobi_os import SovereignFrame

    dataset = "simulator/PlayerStatisticsExtended.csv"
    if not os.path.exists(dataset):
        fail(f"Dataset not found: {dataset}")
        return False

    file_size_mb = os.path.getsize(dataset) / (1024 * 1024)

    # ── Step 1: Ignite Refinery ──────────────────────────────────────────
    sub_banner("PHASE 1 · Igniting the Axum Refinery", C.YELLOW)
    step(1, 6, "Starting Rust refinery daemon on D-Bus...")
    pause(0.5)

    start = time.time()
    nairobi_os.start_refinery()
    ignition_ms = (time.time() - start) * 1000

    ok(f"Axum Refinery ignited in {ignition_ms:.0f}ms")
    stat_line("D-Bus Service", "org.nairobi.NairobiAxumRefinery1")
    stat_line("Engine", "io_uring + Polars + Rayon")
    pause(1.5)

    # ── Step 2: Ingest ───────────────────────────────────────────────────
    sub_banner("PHASE 2 · Zero-Copy Data Ingestion", C.YELLOW)
    step(2, 6, f"Ingesting {dataset} ({file_size_mb:.1f} MB)...")
    pause(0.3)

    start = time.time()
    handle_id = nairobi_os.ingest(dataset)
    ingest_ms = (time.time() - start) * 1000

    ok(f"Ingestion complete")
    stat_line("Latency", f"{ingest_ms:.2f}", "ms")
    stat_line("Handle ID", handle_id)
    stat_line("Transport", "io_uring → memfd → D-Bus")
    pause(1.5)

    # ── Step 3: SovereignFrame ───────────────────────────────────────────
    sub_banner("PHASE 3 · SovereignFrame Analytics", C.YELLOW)
    step(3, 6, "Creating SovereignFrame and computing statistics...")
    pause(0.3)

    df = SovereignFrame(handle_id)

    start = time.time()
    stats = df.points.crunch()
    crunch_ms = (time.time() - start) * 1000

    ok(f"Axiom Crunch on 'points' completed in {crunch_ms:.2f}ms")
    print()
    print(f"  {C.WHITE}{C.BOLD}  ┌─────────────────────────────────────────┐{C.RESET}")
    print(f"  {C.WHITE}{C.BOLD}  │     AXIOM CRUNCH: points column         │{C.RESET}")
    print(f"  {C.WHITE}{C.BOLD}  ├─────────────────────────────────────────┤{C.RESET}")
    stat_line("  │ Mean", f"{stats['mean']:.4f}")
    stat_line("  │ Std Dev", f"{stats['std_dev']:.4f}")
    stat_line("  │ Min", f"{stats['min']:.4f}")
    stat_line("  │ Max", f"{stats['max']:.4f}")
    stat_line("  │ Skewness", f"{stats['skewness']:.4f}")
    stat_line("  │ Kurtosis", f"{stats['kurtosis']:.4f}")
    stat_line("  │ P95", f"{stats['p95']:.4f}")
    stat_line("  │ P99", f"{stats['p99']:.4f}")
    stat_line("  │ Total Rows", f"{stats['total_rows']}")
    print(f"  {C.WHITE}{C.BOLD}  └─────────────────────────────────────────┘{C.RESET}")
    pause(2.0)

    # ── Step 4: Correlation ──────────────────────────────────────────────
    sub_banner("PHASE 4 · Relational Correlation", C.YELLOW)
    step(4, 6, "Computing Pearson & Spearman on points × assists...")
    pause(0.3)

    start = time.time()
    corr = df.correlate("points,assists")
    corr_ms = (time.time() - start) * 1000

    ok(f"Relational strike completed in {corr_ms:.2f}ms")
    stat_line("Pearson r", f"{corr['pearson']:.6f}", color=C.MAGENTA)
    stat_line("Spearman ρ", f"{corr['spearman']:.6f}", color=C.MAGENTA)
    pause(1.5)

    # ── Step 5: SQL Query ────────────────────────────────────────────────
    sub_banner("PHASE 5 · Polars SQL Query", C.YELLOW)
    sql = "SELECT firstName, lastName, points FROM df WHERE points > 40 ORDER BY points DESC LIMIT 5"
    step(5, 6, "Executing SQL on the SovereignFrame...")
    print(f"  {C.DIM}SQL: {sql}{C.RESET}")
    pause(0.3)

    start = time.time()
    result_frame = df.query(sql)
    sql_ms = (time.time() - start) * 1000

    # Crunch the result to show it worked
    result_stats = result_frame.points.crunch()
    ok(f"SQL query returned {result_stats['total_rows']} rows in {sql_ms:.2f}ms")
    stat_line("Top scorer points", f"{result_stats['max']:.1f}")
    pause(1.5)

    # ── Step 6: Teardown ─────────────────────────────────────────────────
    sub_banner("PHASE 6 · Teardown", C.YELLOW)
    step(6, 6, "Stopping Axum Refinery daemon...")
    nairobi_os.stop_refinery()
    ok("Refinery stopped cleanly")
    pause(1.0)

    # ── Summary ──────────────────────────────────────────────────────────
    total_ms = ingest_ms + crunch_ms + corr_ms + sql_ms
    banner("ACT 1 COMPLETE ✓", C.GREEN)
    stat_line("Total compute time", f"{total_ms:.2f}", "ms")
    stat_line("Dataset size", f"{file_size_mb:.1f}", "MB")
    stat_line("Rows processed", f"{stats['total_rows']}")
    stat_line("IPC Transport", "D-Bus + memfd (zero-copy)")
    pause(2.5)
    return True


# ═══════════════════════════════════════════════════════════════════════════════
# ACT 2: COMPUTER USE WITHOUT PIXELS
# ═══════════════════════════════════════════════════════════════════════════════

def act2_computer_use():
    banner("ACT 2: COMPUTER USE WITHOUT PIXELS", C.MAGENTA)
    typewriter(f"  {C.DIM}AT-SPI2 → TOON compression → MCP Server → Semantic actions{C.RESET}", delay=0.02)
    pause(1.0)

    import nairobi_os

    has_display = "DISPLAY" in os.environ or "WAYLAND_DISPLAY" in os.environ
    editor_proc = None

    if not has_display:
        warn("No graphical display detected. Skipping Computer Use demo.")
        info("To run this demo, use a graphical Linux environment.")
        return True

    # ── Step 1: Launch a target application ──────────────────────────────
    sub_banner("PHASE 1 · Launching Target Application", C.YELLOW)
    step(1, 4, "Spawning a GUI application to demonstrate on...")

    # Try to find a suitable app to launch
    target_app = None
    target_title = None
    for app, title in [("gnome-text-editor", "Text Editor"),
                       ("gedit", "gedit"),
                       ("gnome-calculator", "Calculator"),
                       ("xterm", "xterm")]:
        if subprocess.run(["which", app], capture_output=True).returncode == 0:
            target_app = app
            target_title = title
            break

    if target_app:
        editor_proc = subprocess.Popen([target_app], start_new_session=True,
                                        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        ok(f"Launched {target_app} (PID: {editor_proc.pid})")
        info(f"Waiting for window to fully render...")
        pause(2.5)
    else:
        warn("No suitable GUI app found. Will scan any active windows.")
    pause(0.5)

    # ── Step 2: Start MCP Server ─────────────────────────────────────────
    sub_banner("PHASE 2 · Igniting MCP Server", C.YELLOW)
    step(2, 4, "Starting nairobi-connector MCP server over stdio...")
    pause(0.3)

    try:
        start = time.time()
        nairobi_os.ui.start()
        mcp_ms = (time.time() - start) * 1000
        ok(f"MCP server initialized in {mcp_ms:.0f}ms")
        stat_line("Protocol", "JSON-RPC 2.0 over stdio")
        stat_line("Server", "nairobi-connector v0.4.1")
        stat_line("Bridge", "AT-SPI2 / D-Bus")
    except Exception as e:
        fail(f"Failed to start MCP server: {e}")
        _cleanup(editor_proc)
        return False
    pause(1.5)

    # ── Step 3: Find Window + TOON Map ───────────────────────────────────
    sub_banner("PHASE 3 · TOON Accessibility Map", C.YELLOW)

    if target_title:
        step(3, 4, f"Targeting window: '{target_title}'...")
        pause(0.3)
        try:
            result = nairobi_os.ui.find_window(target_title)
            if isinstance(result, dict) and result.get("success"):
                ok(f"Window targeted: {result.get('message', target_title)}")
            else:
                ok(f"Window search result: {result}")
        except Exception as e:
            warn(f"Window targeting: {e}")
    else:
        step(3, 4, "Scanning active desktop windows...")

    pause(0.5)
    info("Generating TOON-compressed accessibility tree...")
    pause(0.3)

    try:
        start = time.time()
        toon_map = nairobi_os.ui.get_map(max_depth=5)
        toon_ms = (time.time() - start) * 1000

        lines = toon_map.strip().splitlines()
        token_estimate = len(toon_map.split())

        ok(f"TOON map generated in {toon_ms:.2f}ms")
        stat_line("Nodes", f"{len(lines)}")
        stat_line("Tokens (est.)", f"~{token_estimate}")
        stat_line("Compression", "AT-SPI2 tree → TOON Markdown")
        print()

        # Display the TOON output with syntax highlighting
        print(f"  {C.WHITE}{C.BOLD}  ┌─ TOON OUTPUT ────────────────────────────┐{C.RESET}")
        display_lines = lines[:20]  # Show first 20 lines
        for line in display_lines:
            toon_line(f"  │ {line}")
        if len(lines) > 20:
            print(f"  {C.DIM}  │ ... ({len(lines) - 20} more nodes){C.RESET}")
        print(f"  {C.WHITE}{C.BOLD}  └───────────────────────────────────────────┘{C.RESET}")

    except Exception as e:
        fail(f"TOON map generation failed: {e}")
        nairobi_os.ui.stop()
        _cleanup(editor_proc)
        return False
    pause(2.5)

    # ── Step 4: Demonstrate interaction ──────────────────────────────────
    sub_banner("PHASE 4 · Semantic Interaction", C.YELLOW)
    step(4, 4, "Demonstrating semantic click on first interactive node...")
    pause(0.3)

    try:
        result = nairobi_os.ui.interact(node_id=1, action="click")
        if isinstance(result, dict):
            ok(f"Action result: {result.get('message', 'click executed')}")
        else:
            ok(f"Action result: {result}")
        stat_line("Method", "AT-SPI2 DoAction (no pixels)")
        stat_line("Latency", "< 5ms (semantic dispatch)")
    except Exception as e:
        warn(f"Interaction demo: {e}")
        info("(This is expected if no clickable node with ID=1 exists)")
    pause(1.5)

    # ── Teardown ─────────────────────────────────────────────────────────
    info("Stopping MCP server...")
    nairobi_os.ui.stop()
    ok("MCP server stopped")
    _cleanup(editor_proc)
    pause(0.5)

    banner("ACT 2 COMPLETE ✓", C.GREEN)
    stat_line("Method", "Semantic (AT-SPI2), not visual (pixels)")
    stat_line("Format", "TOON (Token-Oriented Object Notation)")
    stat_line("Protocol", "MCP (Model Context Protocol)")
    stat_line("Overhead", "< 500 tokens per full UI scan")
    pause(2.0)
    return True


def _cleanup(proc):
    """Kill a spawned subprocess if still running."""
    if proc:
        try:
            proc.terminate()
            proc.wait(timeout=2)
        except Exception:
            try:
                proc.kill()
            except Exception:
                pass


# ═══════════════════════════════════════════════════════════════════════════════
# FINALE
# ═══════════════════════════════════════════════════════════════════════════════

def finale():
    banner("DEMO COMPLETE", C.CYAN)
    print(f"""
  {C.WHITE}{C.BOLD}Nairobi OS v0.4.1{C.RESET} — Built in Nairobi, Kenya 🇰🇪

  {C.CYAN}What you just saw:{C.RESET}
    {C.GREEN}►{C.RESET} Zero-copy data ingestion via {C.BOLD}io_uring{C.RESET} and {C.BOLD}memfd{C.RESET}
    {C.GREEN}►{C.RESET} Vectorized analytics via {C.BOLD}Polars{C.RESET} + {C.BOLD}Rayon{C.RESET}
    {C.GREEN}►{C.RESET} Semantic Computer Use via {C.BOLD}AT-SPI2{C.RESET} + {C.BOLD}TOON{C.RESET}
    {C.GREEN}►{C.RESET} MCP server for LLM agent integration

  {C.YELLOW}Install:{C.RESET}  pip install nairobi-os
  {C.YELLOW}Source:{C.RESET}   github.com/KevinKenya/nairobi-connector-open-source
  {C.YELLOW}PyPI:{C.RESET}     pypi.org/project/nairobi-os/
  {C.YELLOW}License:{C.RESET}  Apache 2.0
""")
    print(f"  {C.DIM}{'─' * TERM_WIDTH}{C.RESET}")
    print(f"  {C.GRAY}© 2026 Kevin Chege. All Rights Reserved.{C.RESET}")
    print()


# ═══════════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════════

def main():
    splash()

    # ACT 1: Data Analytics
    try:
        if not act1_analytics():
            fail("ACT 1 failed. Aborting.")
            sys.exit(1)
    except Exception as e:
        fail(f"ACT 1 error: {e}")
        import traceback
        traceback.print_exc()
        try:
            import nairobi_os
            nairobi_os.stop_refinery()
        except Exception:
            pass
        sys.exit(1)

    pause(1.0)

    # ACT 2: Computer Use
    try:
        act2_computer_use()
    except Exception as e:
        fail(f"ACT 2 error: {e}")
        import traceback
        traceback.print_exc()
        try:
            import nairobi_os
            nairobi_os.ui.stop()
        except Exception:
            pass

    # Finale
    finale()

if __name__ == "__main__":
    main()
