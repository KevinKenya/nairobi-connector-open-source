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
Nairobi OS - Visual DAG Demo with Automatic Execution

Opens the Canvas native UI pre-populated with a PlayerStatistics pipeline
that auto-executes on compile, then displays the generated PNG.
"""

import json
import os
from pathlib import Path

import nairobi_os


def main():
    print("=" * 60)
    print("Nairobi OS - Visual DAG Demo")
    print("=" * 60)
    print()

    lagos_bin = Path("/home/chege/nairobi-connector-open-source/.venv/lib/python3.12/site-packages/nairobi_os/bin/lagos-vision-daemon")
    os.environ["LAGOS_VISION_DAEMON_BIN"] = str(lagos_bin)

    print("## Starting Nairobi Infrastructure")
    try:
        nairobi_os.ignite()
        print("  ✓ Hub daemon started")
    except RuntimeError as e:
        print(f"  ✓ Hub already running: {e}")
    print()

    print("## Opening Canvas with Player Statistics Preset")
    print("  Preset: playerstats_vis (Ingest -> SqlQuery -> LagosPlot)")
    print("  Auto-execution: enabled")
    print()

    # Open canvas with preset and auto-execution
    result = nairobi_os._core.canvas.open("playerstats_vis", True)
    
    if result is None:
        print("  Canvas was cancelled or compilation failed")
        return

    print(f"  ✓ DAG compiled: {len(result)} bytes")
    print()

    print("## Verifying Output")

    output_path = "/tmp/lagos-output-2.png"
    if Path(output_path).exists():
        size = Path(output_path).stat().st_size
        print(f"  ✓ Plot saved to {output_path}")
        print(f"  ✓ Output file size: {size} bytes")

        try:
            from PIL import Image
            img = Image.open(output_path)
            print(f"  ✓ Image dimensions: {img.width}x{img.height}px")
            print()
            try:
                img.show()
            except Exception as e:
                print(f"  Note: Could not open image viewer ({e}). Image saved at: {output_path}")
        except ImportError:
            print("  Note: PIL not available, cannot display image")
    else:
        print(f"  ✗ Expected output not found at {output_path}")

    print()
    print("✓ Visual DAG demo completed!")


if __name__ == "__main__":
    main()