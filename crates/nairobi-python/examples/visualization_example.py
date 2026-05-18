#!/usr/bin/env python3
"""
Nairobi OS Python Visualization Example

This script demonstrates the hardware-accelerated visualization capabilities
of Nairobi OS using Lagos Vision.
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
    
    # Show available visualization functions
    print("## Available Lagos Visualization Functions")
    lagos_functions = [func for func in dir(nairobi_os.lagos) if not func.startswith('_')]
    for func in sorted(lagos_functions):
        print(f"  - {func}")
    print()
    
    # Example usage patterns for visualization
    print("## Visualization Usage Patterns (Conceptual)")
    print("""
    # Start the refinery daemon
    nairobi_os.start_refinery()
    
    # Ingest a CSV file
    handle_id = nairobi_os.data.ingest('data.csv')
    
    # Compute statistical moments
    stats_json = nairobi_os.data.crunch(handle_id, 'column_name')
    stats = json.loads(stats_json)
    print(f"Mean: {stats['mean']:.4f}")
    
    # Create interactive visualization (in Jupyter)
    # This creates a hardware-accelerated plot using Lagos Vision
    # widget = nairobi_os.lagos.plot_inline(handle_id='abc-123', width=1000, height=400)
    # display(widget)  # In Jupyter notebook
    
    # Or create a static plot
    # plot_data = nairobi_os.lagos.create_plot(handle_id, 'column_name', plot_type='line')
    
    # Cleanup
    nairobi_os.data.free(handle_id)
    # nairobi_os.stop_refinery()  # Optional
    """)
    
    print("\n✓ Visualization example script completed!")

if __name__ == "__main__":
    main()