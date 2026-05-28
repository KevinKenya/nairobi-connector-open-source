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
Nairobi OS Python - Hardware-Accelerated Visualization

Lagos Vision integration:
- Create interactive plots
- Jupyter widget integration
- Visualization parameters
"""

import nairobi_os

def main():
    print(f"Nairobi OS version: {nairobi_os.__version__}")
    print()

    # Show available lagos functions
    print("## Available Lagos Visualization Functions")
    lagos_functions = [func for func in dir(nairobi_os.lagos) if not func.startswith('_')]
    for func in sorted(lagos_functions):
        print(f"  - {func}")
    print()

    print("## Lagos Visualization Usage (Conceptual)")
    print()
    print("    # Start the Refinery daemon")
    print("    nairobi_os.start_refinery()")
    print("    print('✓ Refinery daemon started!')")
    print()
    print("    # Ingest data")
    print("    handle_id = nairobi_os.data.ingest('time_series_data.csv')")
    print()

    # Interactive plots
    print("=== Interactive Plots (Jupyter) ===")
    print("    # Create interactive visualization using wgpu hardware acceleration")
    print("    # This creates a Jupyter widget for interactive exploration")
    print()
    print("    # In Jupyter notebook:")
    print("    # widget = nairobi_os.lagos.plot_inline(handle_id=handle_id)")
    print("    # display(widget)")
    print()

    # Plotting functions
    print("=== Plot Creation ===")
    print("    # Static plot creation (returns image data or file path)")
    print("    # plot_data = nairobi_os.lagos.create_plot(handle_id, 'revenue', 'line')")
    print()

    # SovereignFrame integration
    print("=== SovereignFrame Plot Integration ===")
    print("    # Use the SovereignFrame's built-in plot method")
    print("    frame = nairobi_os.SovereignFrame(handle_id)")
    print("    # widget = frame.plot(width=1000, height=400)")
    print()

    # Visualization parameters
    print("=== Visualization Parameters ===")
    print("    # plot_inline accepts:")
    print("    #   - handle_id: The data handle")
    print("    #   - width: Plot width in pixels (default: 800)")
    print("    #   - height: Plot height in pixels (default: 400)")
    print()

    # Cleanup
    print("=== Cleanup ===")
    print("    nairobi_os.data.free(handle_id)")
    print("    # nairobi_os.stop_refinery()  # Optional")

    print("\n✓ Lagos visualization demo completed!")

if __name__ == "__main__":
    main()