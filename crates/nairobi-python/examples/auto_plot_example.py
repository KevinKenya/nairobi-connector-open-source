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
Nairobi OS - Programmatic DAG Construction and Headless Plot Execution

Demonstrates building and executing a DAG (Ingest -> SqlQuery -> LagosPlot)
headlessly without opening the Canvas UI, rendering to a PNG file.
"""

import json
import os
import nairobi_os
from pathlib import Path


def main():
    print("=" * 60)
    print("Nairobi OS - Auto Plot Example")
    print("=" * 60)
    print()

    dataset_path = Path(__file__).resolve().parents[2] / "simulator" / "PlayerStatisticsExtended.csv"
    
    # Set the lagos-vision-daemon path for the executor BEFORE starting Hub
    lagos_bin = Path(nairobi_os.__file__).parent / "bin" / "lagos-vision-daemon"
    os.environ["LAGOS_VISION_DAEMON_BIN"] = str(lagos_bin)

    print(f"Dataset: {dataset_path}")
    print()

    # Start Hub daemon BEFORE setting env var so it inherits the environment
    print("## Starting Nairobi Infrastructure")
    try:
        nairobi_os.ignite()
        print("  ✓ Hub daemon started")
    except RuntimeError as e:
        print(f"  ✓ Hub already running: {e}")
    print()

    # Build DAG programmatically: Ingest -> SqlQuery -> LagosPlot
    nodes = [
        (0, "Ingest", json.dumps({"dataset_path": str(dataset_path)})),
        (1, "SqlQuery", json.dumps({"query": "SELECT points FROM dataset"})),
        (2, "LagosPlot", json.dumps({"format": "png", "width": 1200, "height": 400})),
    ]

    edges = [
        (0, 1),  # Ingest output -> SqlQuery input
        (1, 2),  # SqlQuery output -> LagosPlot input
    ]

    print("## Building DAG from configuration")
    print(f"  Nodes: {len(nodes)} (Ingest -> SqlQuery -> LagosPlot)")
    print(f"  Edges: {len(edges)}")
    print()

    dag_bytes = nairobi_os._core.canvas.build_dag(nodes, edges)
    print(f"  Compiled DAG size: {len(dag_bytes)} bytes")
    print()

    # Execute the DAG via Hub
    print("## Executing DAG")
    nairobi_os._core.canvas.execute(dag_bytes)

    print()
    print("## Verifying Output")
    
    for node in nodes:
        if node[1] == "LagosPlot":
            node_format = json.loads(node[2]).get("format", "sparkline")
            ext = "png" if node_format == "png" else "jpg"
            expected_output = f"/tmp/lagos-output-{node[0]}.{ext}"
            if Path(expected_output).exists():
                print(f"  ✓ Plot saved to {expected_output}")
                size = Path(expected_output).stat().st_size
                print(f"  ✓ Output file size: {size} bytes")
            else:
                print(f"  ✗ Expected output not found at {expected_output}")

    print()
    print("✓ Auto plot example completed!")


if __name__ == "__main__":
    main()