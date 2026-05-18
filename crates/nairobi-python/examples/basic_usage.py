#!/usr/bin/env python3
"""
Nairobi OS Python Basic Usage Example

This script demonstrates basic usage of the Nairobi OS Python bindings for 
high-performance data processing.
"""

import nairobi_os
import json
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
    
    # For demonstration purposes, we'll show how the API works
    # without actually processing a file (since we don't have sample data)
    print("## API Demonstration")
    print("Available functions in nairobi_os.data:")
    data_functions = [func for func in dir(nairobi_os.data) if not func.startswith('_')]
    for func in sorted(data_functions):
        print(f"  - {func}")
    print()
    
    print("Available functions in nairobi_os.lagos:")
    lagos_functions = [func for func in dir(nairobi_os.lagos) if not func.startswith('_')]
    for func in sorted(lagos_functions):
        print(f"  - {func}")
    print()
    
    # Show high-level interface
    print("## High-Level Interface")
    print("nairobi_os.SovereignFrame - Provides pandas-like interface")
    print("nairobi_os.lagos.plot_inline - Creates interactive Jupyter widgets")
    print()
    
    # Example usage patterns (commented out since we don't have actual data)
    print("## Example Usage Patterns (Conceptual)")
    print("""
    # Start the refinery daemon
    nairobi_os.start_refinery()
    
    # Ingest a CSV file
    handle_id = nairobi_os.data.ingest('data.csv')
    
    # Compute statistical moments
    stats_json = nairobi_os.data.crunch(handle_id, 'column_name')
    stats = json.loads(stats_json)
    print(f"Mean: {stats['mean']:.4f}")
    
    # Compute correlation
    corr_json = nairobi_os.data.correlate(handle_id, 'col1,col2')
    corr = json.loads(corr_json)
    print(f"Correlation: {corr['pearson']:.4f}")
    
    # Use the fused pipeline for maximum performance
    result = nairobi_os.data.pipeline(
        'data.csv', 
        'value_column', 
        'col1,col2'
    )
    result_data = json.loads(result)
    
    # High-level interface
    frame = nairobi_os.SovereignFrame(handle_id)
    mean_value = frame.value_column.mean()
    
    # Create interactive visualization (in Jupyter)
    # widget = nairobi_os.lagos.plot_inline(handle_id='abc-123')
    
    # Cleanup
    nairobi_os.data.free(handle_id)
    # nairobi_os.stop_refinery()  # Optional
    """)
    
    print("\n✓ Example script completed successfully!")

if __name__ == "__main__":
    main()