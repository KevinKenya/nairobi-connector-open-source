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
Nairobi OS Python - SovereignFrame Pandas-Like Interface

SovereignFrame high-level API:
- Column accessor fluent API
- DataFrame-like operations
- SQL querying with query() method
- Integration with pandas/numpy
"""

import nairobi_os
import json

def main():
    print(f"Nairobi OS version: {nairobi_os.__version__}")
    print()

    # Show available high-level functions
    print("## Available High-Level Functions")
    print("  - nairobi_os.SovereignFrame")
    print("  - nairobi_os.ColumnAccessor")
    print()

    print("## SovereignFrame Usage (Conceptual)")
    print()
    print("    # Start the Refinery daemon")
    print("    nairobi_os.start_refinery()")
    print("    print('✓ Refinery daemon started successfully!')")
    print()
    print("    # Ingest data and create a SovereignFrame")
    print("    handle_id = nairobi_os.data.ingest('sales_data.csv')")
    print()
    print("    # Create high-level frame (like a pandas DataFrame)")
    print("    frame = nairobi_os.SovereignFrame(handle_id)")
    print()

    # Column accessor examples
    print("\n=== Column Accessor Fluent API ===")
    print("    # Access columns via attribute (fluent API like pandas)")
    print("    mean_value = frame.revenue.mean()")
    print("    max_value = frame.revenue.max()")
    print("    std_value = frame.revenue.std_dev()")
    print("    p95_value = frame.revenue.p95()")
    print("    skew_value = frame.revenue.skewness()")
    print()

    # SQL querying
    print("=== SQL Querying with SovereignFrame ===")
    print("    # SQL-like querying")
    print("    filtered_frame = frame.query('SELECT * FROM data WHERE revenue > 1000')")
    print()

    # Correlation
    print("=== Correlation ===")
    print("    # Compute correlation matrix")
    print("    corr_matrix = frame.correlate('revenue,ad_spend,leads')")
    print()

    # Integration with pandas/numpy
    print("=== Integration with pandas/numpy ===")
    print("    try:")
    print("        import pandas as pd")
    print("        # stats_df = pd.Series(frame.revenue.crunch())")
    print("    except ImportError:")
    print("        print('pandas not available - install with: pip install pandas')")
    print()

    # Cleanup
    print("=== Cleanup ===")
    print("    frame.free()")
    print("    # nairobi_os.stop_refinery()  # Optional")

    print("\n✓ SovereignFrame pandas-like demo completed!")

if __name__ == "__main__":
    main()