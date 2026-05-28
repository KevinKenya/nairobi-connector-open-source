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
Nairobi OS Python - Fused Analytics Pipeline

Maximum performance path:
- Single-call ingest → crunch → correlate
- Compare fused vs separate calls
- Performance profiling
"""

import nairobi_os
import json
import time

def main():
    print(f"Nairobi OS version: {nairobi_os.__version__}")
    print()

    # Show available pipeline functions
    print("## Available Pipeline Functions")
    print("  - nairobi_os.pipeline()")
    print("  - nairobi_os.data.crunch_and_correlate()")
    print()

    print("## Fused Analytics Pipeline Workflow (Conceptual)")
    print()
    print("    # Start the Refinery daemon")
    print("    nairobi_os.start_refinery()")
    print("    print('✓ Refinery daemon started successfully!')")
    print()
    
    print("=== Fused Pipeline API Pattern ===")
    print("    # Single call: ingest + crunch + correlate")
    print("    result = nairobi_os.pipeline(")
    print("        'sales_data.csv',           # Input file")
    print("        'revenue',                  # Column to crunch")
    print("        'ad_spend,sales,leads',    # Correlation columns")
    print("        delimiter=',',              # CSV delimiter (optional)")
    print("        encoding='utf-8'            # File encoding (optional)")
    print("    )")
    print()
    print("    result_data = json.loads(result)")
    print("    print(f'Result keys: {list(result_data.keys())}')")
    
    print("\n=== Performance Comparison ===")
    print("    # Method 1: Separate calls (more round-trips)")
    print("    start = time.time()")
    print("    # handle = nairobi_os.data.ingest('large_dataset.csv')")
    print("    # crunch_stats = json.loads(nairobi_os.data.crunch(handle, 'revenue'))")
    print("    # corr_stats = json.loads(nairobi_os.data.correlate(handle, 'col1,col2'))")
    print("    separate_time = time.time() - start")
    print()
    print("    # Method 2: Fused pipeline (single call)")
    print("    start = time.time()")
    print("    # result = nairobi_os.pipeline('large_dataset.csv', 'revenue', 'col1,col2')")
    print("    fused_time = time.time() - start")
    print("    print(f'Speedup: {separate_time / fused_time:.2f}x faster')")
    
    print("\n=== Crunch and Correlate ===")
    print("    # handle = nairobi_os.data.ingest('data.csv')")
    print()
    print("    # Combined crunch and correlate in one call")
    print("    combined = nairobi_os.data.crunch_and_correlate(handle, 'value_column', 'col1,col2,col3')")
    print("    combined_data = json.loads(combined)")
    print()
    print("    print(f'Crunch stats available: {list(combined_data.get(\"crunch\", {}).keys())}')")
    print("    print(f'Correlation matrix available: {list(combined_data.get(\"correlation\", {}).keys())}')")
    
    print("\n=== Cleanup ===")
    print("    # fused pipeline handles cleanup internally")
    print("    # nairobi_os.stop_refinery()  # Optional")

    print("\n✓ Fused analytics pipeline demo completed!")

if __name__ == "__main__":
    main()