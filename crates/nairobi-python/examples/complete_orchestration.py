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
Nairobi OS Python - Full Platform Orchestration

End-to-end workflow combining all components:
- Start both Refinery and Hub daemons
- Process data with fused pipeline
- Visualize results
- Clean shutdown
"""

import nairobi_os
import json

def main():
    print(f"Nairobi OS version: {nairobi_os.__version__}")
    print()

    print("## Complete Platform Orchestration (Conceptual)")
    print()

    # Step 1: Start both daemons
    print("=== Step 1: Start Both Daemons ===")
    print("    nairobi_os.ignite()  # Starts Refinery + Hub")
    print("    print('✓ Both daemons started!')")
    print()

    # Step 2: Data processing
    print("=== Step 2: Data Processing ===")
    print("    result = nairobi_os.pipeline('business_data.csv', 'revenue', 'ad_spend,leads,sales')")
    print("    result_data = json.loads(result)")
    print("    crunch_stats = result_data.get('crunch_stats', {})")
    print()

    # Step 3: Analyze results
    print("=== Step 3: Analyze Results ===")
    print("    # Extract crunch statistics")
    print("    print(f\"Mean: {crunch_stats.get('mean', 0):.2f}\")")
    print("    print(f\"Std Dev: {crunch_stats.get('std_dev', 0):.2f}\")")
    print("    print(f\"95th Percentile: {crunch_stats.get('p95', 0):.2f}\")")
    print()

    # Step 4: Visualization
    print("=== Step 4: Visualization ===")
    print("    # Create interactive visualization with Lagos Vision (Jupyter)")
    print("    # widget = nairobi_os.lagos.plot_inline(handle_id='handle_id')")
    print()

    # Step 5: UI automation
    print("=== Step 5: UI Automation ===")
    print("    # with nairobi_os.ui as u:")
    print("    #     u.find_window('Business Reports')")
    print("    #     u.interact(node_id=5, action='click')")
    print()

    # Step 6: Canvas integration
    print("=== Step 6: Canvas Integration ===")
    print("    # nairobi_os.canvas.open()")
    print("    # dag_result = nairobi_os.canvas.execute(dag_id='sales_pipeline')")
    print()

    # Cleanup
    print("=== Cleanup ===")
    print("    # nairobi_os.stop_refinery()")

    print("\n✓ Complete orchestration demo completed!")

if __name__ == "__main__":
    main()