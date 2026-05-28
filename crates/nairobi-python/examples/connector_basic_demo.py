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
Nairobi OS Python - Basic MCP Connector Demo

Demonstrates the core MCP tools from Nairobi Connector:
- Starting the MCP server
- Finding windows by title
- Getting TOON UI maps
- Basic interactions (click, focus)
"""

import nairobi_os

def main():
    print(f"Nairobi OS version: {nairobi_os.__version__}")
    print()

    # Show available UI functions
    print("## Available MCP Connector Functions")
    ui_functions = [func for func in dir(nairobi_os.ui) if not func.startswith('_')]
    for func in sorted(ui_functions):
        print(f"  - {func}")
    print()

    print("## MCP Connector Usage Patterns (Conceptual)")
    print()
    print("    # Start the MCP server (auto-starts on first use)")
    print("    nairobi_os.ui.start()")
    print("    print('✓ MCP server started!')")
    print()
    print("    # Find a window by title")
    print("    result = nairobi_os.ui.find_window('Terminal')")
    print()
    print("    # Get the UI accessibility tree as TOON format")
    print("    toon_map = nairobi_os.ui.get_map()")
    print("    print(toon_map)")
    print()
    print("    # Interact with UI elements by node ID")
    print("    click_result = nairobi_os.ui.interact(node_id=3, action='click')")
    print("    focus_result = nairobi_os.ui.interact(node_id=5, action='focus')")
    print("    activate_result = nairobi_os.ui.interact(node_id=2, action='activate')")
    print()
    print("    # Check server status")
    print("    is_running = nairobi_os.ui.is_running()")
    print("    print(f'Server running: {is_running}')")
    print()
    print("    # Stop the MCP server")
    print("    nairobi_os.ui.stop()")
    print("    print('✓ MCP server stopped!')")

    print("\n✓ Connector basic demo script completed!")

if __name__ == "__main__":
    main()