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
Nairobi OS Python - UI Automation Workflow

Complete desktop automation workflow:
- Target a browser or text editor
- Navigate UI via TOON tree
- Fill forms and click buttons
- Extract text from document elements
"""

import nairobi_os
import time

def main():
    print(f"Nairobi OS version: {nairobi_os.__version__}")
    print()

    # Show available UI functions for automation
    print("## UI Automation Functions")
    ui_functions = [func for func in dir(nairobi_os.ui) if not func.startswith('_')]
    for func in sorted(ui_functions):
        print(f"  - {func}")
    print()

    print("## UI Automation Workflow (Conceptual)")
    print()
    print("    # Start the MCP server using context manager for automatic cleanup")
    print("    with nairobi_os.ui as u:")
    print("        print('✓ MCP server started!')")
    print()
    print("        # Step 1: Target a browser window")
    print("        result = u.find_window('Firefox')")
    print()
    print("        # Step 2: Get the accessibility tree as TOON format")
    print("        toon_map = u.get_map(max_depth=7)")
    print("        print(toon_map)")
    print()
    print("        # Step 3: Navigate and interact with UI elements")
    print("        # u.interact(node_id=4, action='click')  # Click search box")
    print("        # u.type_text(node_id=4, text='Nairobi OS Python examples')")
    print()
    print("        # Step 4: Work with a text editor")
    print("        # u.find_window('Text Editor')")
    print("        # u.interact(node_id=3, action='click')")
    print("        # u.type_text(node_id=3, text='Hello from Nairobi OS!')")
    print()
    print("        # Step 5: Form automation")
    print("        # u.find_window('Login Form')")
    print("        # u.interact(node_id=2, action='click')")
    print("        # u.type_text(node_id=2, text='user@example.com')")
    print("        # u.interact(node_id=5, action='click')  # Submit")
    print()
    print("    print('✓ MCP server stopped (via context manager)!')")

    print("\n✓ UI automation demo script completed!")

if __name__ == "__main__":
    main()