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
Nairobi OS Python - Node Graph Compilation

Canvas visual compiler:
- Open the canvas UI
- Compile node graphs to GVariant DAG
- Execute compiled DAGs via Hub
"""

import nairobi_os

def main():
    print(f"Nairobi OS version: {nairobi_os.__version__}")
    print()

    # Show available canvas functions
    print("## Available Canvas Functions")
    canvas_functions = [func for func in dir(nairobi_os.canvas) if not func.startswith('_')]
    for func in sorted(canvas_functions):
        print(f"  - {func}")
    print()

    print("## Canvas DAG Compilation Workflow (Conceptual)")
    print()
    print("    # Start the Hub daemon (required for Canvas)")
    print("    nairobi_os.ignite()")
    print("    print('✓ Hub daemon started successfully!')")
    print()

    # Opening canvas UI
    print("=== Opening Canvas UI ===")
    print("    # Open the canvas UI for visual node graph construction")
    print("    # canvas_window = nairobi_os.canvas.open()")
    print("    # The canvas provides a visual interface for constructing")
    print("    # data processing pipelines using drag-and-drop nodes")
    print()

    # Node graph compilation
    print("=== Node Graph Compilation ===")
    print("    # Build a node graph visually in the Canvas UI:")
    print("    # 1. Add Ingest node -> configure file path")
    print("    # 2. Add Crunch node -> select target column")
    print("    # 3. Add Correlate node -> select correlation columns")
    print("    # 4. Connect nodes to form a pipeline")
    print()
    print("    # Compile the visual graph to a GVariant DAG")
    print("    # dag_result = nairobi_os.canvas.execute(dag_name='my_pipeline')")
    print()

    # Executing compiled DAGs
    print("=== Executing Compiled DAGs ===")
    print("    # Execute the compiled DAG via Hub")
    print("    # execution_result = nairobi_os.canvas.execute(dag_id=dag_id)")
    print("    # result contains crunch_stats, correlation, output_handle")
    print()

    # Canvas use cases
    print("=== Canvas Use Cases ===")
    print("    # Use Canvas for complex pipelines:")
    print("    #   - ETL workflows with multiple transformations")
    print("    #   - Multi-step feature engineering")
    print("    #   - Visual data exploration")
    print("    #   - Reusable pipeline templates")
    print()

    # Cleanup
    print("=== Cleanup ===")
    print("    # nairobi_os.stop_hub()  # Cleanup handled automatically")

    print("\n✓ Canvas DAG compilation demo completed!")

if __name__ == "__main__":
    main()