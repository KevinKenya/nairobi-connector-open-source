#!/usr/bin/env python3
"""
Nairobi OS Python Advanced Usage Example

This script demonstrates advanced usage patterns of the Nairobi OS Python bindings,
including the fused analytics pipeline, high-level SovereignFrame interface,
and integration with pandas/numpy workflows.
"""

import nairobi_os
import json
import pandas as pd
import numpy as np
import time

def main():
    print(f"Nairobi OS version: {nairobi_os.__version__}")
    print(f"Author: {nairobi_os.__author__}")
    print()
    
    # Starting the Refinery Daemon
    print("## Starting the Refinery Daemon")
    nairobi_os.start_refinery()
    print("✓ Refinery daemon started successfully!")
    print()
    
    # Show available functions
    print("## Available Data Functions")
    data_functions = [func for func in dir(nairobi_os) if not func.startswith('_') and func not in ['Path', 'logger', 'logging', 'os', 'subprocess', 'time', 'start_refinery', 'stop_refinery']]
    for func in sorted(data_functions):
        print(f"  - {func}")
    print()
    
    print("## Available High-Level Interface")
    print("  - SovereignFrame: Pandas-like interface for data handles")
    print("  - ColumnAccessor: Fluent column access (df.column.mean())")
    print()
    
    # Example usage patterns for advanced features
    print("## Advanced Usage Patterns (Conceptual)")
    print("""
    # Start the refinery daemon
    nairobi_os.start_refinery()
    
    # === FUSED ANALYTICS PIPELINE ===
    # Ingest + Statistics + Correlation in one call (maximum performance)
    result = nairobi_os.pipeline(
        'large_dataset.csv',      # Input file
        'target_column',          # Column to analyze
        'feature1,feature2,feature3',  # Columns to correlate
        delimiter=',',            # CSV delimiter (optional)
        encoding='utf-8'          # File encoding (optional)
    )
    result_data = json.loads(result)
    print(f"Pipeline results keys: {list(result_data.keys())}")
    
    # === HIGH-LEVEL INTERFACE (SOVEREIGN FRAME) ===
    # Ingest data and get a handle
    handle_id = nairobi_os.ingest('large_dataset.csv')
    
    # Create high-level frame (like a DataFrame)
    frame = nairobi_os.SovereignFrame(handle_id)
    
    # Fluent API for column operations
    mean_value = frame.target_column.mean()
    std_value = frame.target_column.std_dev()
    max_value = frame.target_column.max()
    min_value = frame.target_column.min()
    
    # Access multiple columns
    correlation_matrix = frame.correlate('feature1,feature2,feature3')
    
    # SQL-like querying
    new_handle = frame.query("SELECT * FROM data WHERE value > 100")
    filtered_frame = nairobi_os.SovereignFrame(new_handle)
    
    # === VISUALIZATION INTEGRATION ===
    # Hardware-accelerated plotting (in Jupyter)
    # widget = nairobi_os.lagos.plot_inline(handle_id='handle_id', width=1200, height=600)
    
    # === MEMORY MANAGEMENT ===
    # Explicitly free handles when done
    nairobi_os.free(handle_id)
    nairobi_os.free(new_handle)
    
    # Optional: Stop the refinery daemon
    # nairobi_os.stop_refinery()
    """)
    
    print("\n✓ Advanced example script completed!")

if __name__ == "__main__":
    main()