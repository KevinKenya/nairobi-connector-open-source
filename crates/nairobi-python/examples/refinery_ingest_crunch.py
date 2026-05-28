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
Nairobi OS Python - Data Ingestion & Statistics

Data processing with Axum Refinery:
- Ingest CSV data
- Compute statistical moments (mean, std_dev, p95, p99, skewness, kurtosis)
- Anomaly detection
- Memory-efficient handle management
"""

import nairobi_os
import json

def main():
    print(f"Nairobi OS version: {nairobi_os.__version__}")
    print()

    # Show available data functions
    print("## Available Data Functions")
    data_functions = [func for func in dir(nairobi_os.data) if not func.startswith('_')]
    for func in sorted(data_functions):
        print(f"  - {func}")
    print()

    print("## Data Ingestion & Statistics Workflow (Conceptual)")
    print()
    print("    # Start the Refinery daemon")
    print("    nairobi_os.start_refinery()")
    print("    print('✓ Refinery daemon started successfully!')")
    print()
    print("    # Ingest a CSV file")
    print("    # handle_id = nairobi_os.data.ingest('sales_data.csv')")
    print()
    
    print("\n=== Statistical Moments Available ===")
    print("  - mean: Arithmetic mean")
    print("  - max: Maximum value")
    print("  - min: Minimum value")
    print("  - std_dev: Standard deviation")
    print("  - variance: Variance")
    print("  - p95: 95th percentile")
    print("  - p99: 99th percentile")
    print("  - skewness: Distribution skewness")
    print("  - kurtosis: Distribution kurtosis")
    print("  - count: Number of records")
    
    print("\n=== Crunch API Pattern ===")
    print("    crunch_result = nairobi_os.data.crunch(handle_id, 'revenue')")
    print("    stats = json.loads(chunch_result)")
    print()
    print("    print(f'Mean revenue: {stats[\"mean\"]:.2f}')")
    print("    print(f'Max revenue: {stats[\"max\"]:.2f}')")
    print("    print(f'Min revenue: {stats[\"min\"]:.2f}')")
    print("    print(f'Std Dev: {stats[\"std_dev\"]:.2f}')")
    print("    print(f'95th Percentile: {stats[\"p95\"]:.2f}')")
    print("    print(f'Skewness: {stats[\"skewness\"]:.4f}')")
    print("    print(f'Kurtosis: {stats[\"kurtosis\"]:.4f}')")
    
    print("\n=== Anomaly Detection ===")
    print("    # Anomaly detection uses statistical thresholds")
    print("    stats = json.loads(nairobi_os.data.crunch(handle_id, 'transaction_amount'))")
    print()
    print("    # Values beyond 3 standard deviations are anomalies")
    print("    mean = stats['mean']")
    print("    std_dev = stats['std_dev']")
    print()
    print("    # You can filter and query for anomalies")
    print("    # anomaly_handle = nairobi_os.data.sql_query(")
    print("    #     handle_id,")
    print("    #     f'SELECT * FROM dataset WHERE transaction_amount > {mean + 3 * std_dev}'")
    print("    # )")
    
    print("\n=== Memory Management ===")
    print("    # Free handles when no longer needed")
    print("    nairobi_os.data.free(handle_id)")
    print()
    print("    # Optional: Stop the refinery daemon")
    print("    # nairobi_os.stop_refinery()")

    print("\n✓ Refinery ingest crunch demo completed!")

if __name__ == "__main__":
    main()