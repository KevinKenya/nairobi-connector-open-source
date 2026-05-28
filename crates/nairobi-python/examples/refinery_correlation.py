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
Nairobi OS Python - Correlation Analysis

Pearson and Spearman correlation:
- Compute correlation between column pairs
- Multiple column correlation analysis
- Interpret correlation results
"""

import nairobi_os
import json

def main():
    print(f"Nairobi OS version: {nairobi_os.__version__}")
    print()

    # Show available correlation functions
    print("## Available Correlation Functions")
    print("  - nairobi_os.data.correlate()")
    print("  - nairobi_os.data.crunch_and_correlate()")
    print()

    print("## Correlation Analysis Workflow (Conceptual)")
    print("""
    # Start the Refinery daemon
    nairobi_os.start_refinery()
    print("✓ Refinery daemon started successfully!")
    
    # Ingest a dataset with multiple numeric columns
    handle_id = nairobi_os.data.ingest("marketing_data.csv")
    print(f"Handle ID: {handle_id}")
    """)
    
    # Correlation API pattern
    print("\n=== Correlation API Pattern ===")
    print('''
    # Two-column correlation
    corr = nairobi_os.data.correlate(handle_id, "ad_spend,sales")
    corr_data = json.loads(corr)
    
    print(f"Pearson correlation: {corr_data['pearson']:.4f}")
    print(f"Spearman correlation: {corr_data['spearman']:.4f}")
    ''')
    
    # Multiple column correlation
    print("\n=== Multiple Column Correlation ===")
    print('''
    # Three or more columns - gets correlation matrix
    corr = nairobi_os.data.correlate(handle_id, "ad_spend,sales,leads,revenue")
    corr_matrix = json.loads(corr)
    
    # The result is a matrix of pairwise correlations
    for pair, values in corr_matrix.items():
        print(f"{pair}: pearson={values['pearson']:.4f}, spearman={values['spearman']:.4f}")
    ''')

    # Interpretation guide
    print("\n=== Correlation Interpretation ===")
    print('''
    # Pearson correlation (-1 to 1):
    #   - Values near 1: Strong positive linear relationship
    #   - Values near -1: Strong negative linear relationship
    #   - Values near 0: Weak or no linear relationship
    
    # Spearman correlation (-1 to 1):
    #   - Rank-based correlation (monotonic relationships)
    #   - More robust to outliers than Pearson
    
    if corr_data['pearson'] > 0.7:
        print("Strong positive correlation detected!")
    elif corr_data['pearson'] < -0.7:
        print("Strong negative correlation detected!")
    else:
        print("Weak or no linear correlation.")
    ''');

    # Cleanup
    print("\n=== Cleanup ===")
    print('''
    nairobi_os.data.free(handle_id)
    # nairobi_os.stop_refinery()  # Optional
    ''')

    print("\n✓ Refinery correlation demo completed!")

if __name__ == "__main__":
    main()