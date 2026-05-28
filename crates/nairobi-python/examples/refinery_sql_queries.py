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
Nairobi OS Python - SQL Querying Data

SQL on ingested data:
- Register dataset for SQL queries
- Execute SELECT statements
- Filter and aggregate data
- Create derived datasets
"""

import nairobi_os
import json

def main():
    print(f"Nairobi OS version: {nairobi_os.__version__}")
    print()

    # Show available SQL functions
    print("## Available SQL Functions")
    print("  - nairobi_os.data.sql_query()")
    print()

    print("## SQL Querying Workflow (Conceptual)")
    print()
    print("    # Start the Refinery daemon")
    print("    nairobi_os.start_refinery()")
    print("    print('✓ Refinery daemon started!')")
    print()
    print("    handle_id = nairobi_os.data.ingest('customer_data.csv')")
    print()

    print("=== SQL Query API Pattern ===")
    print("    # Execute a SELECT query (table name must be 'dataset')")
    print("    new_handle = nairobi_os.data.sql_query(")
    print("        handle_id,")
    print("        \"SELECT * FROM dataset WHERE status = 'active'\"")
    print("    )")
    print()

    print("=== Filtering Data ===")
    print("    active_customers = nairobi_os.data.sql_query(")
    print("        handle_id,")
    print("        'SELECT customer_id, name, revenue FROM dataset WHERE revenue > 1000'")
    print("    )")
    print()
    print("    premium_customers = nairobi_os.data.sql_query(")
    print("        handle_id,")
    print("        \"SELECT * FROM dataset WHERE tier = 'premium' AND active = 1\"")
    print("    )")
    print()

    print("=== Aggregation ===")
    print("    # Group by and aggregate")
    print("    summary = nairobi_os.data.sql_query(")
    print("        handle_id,")
    print("        'SELECT region, COUNT(*) as count, AVG(revenue) as avg_revenue FROM dataset GROUP BY region'")
    print("    )")
    print()

    print("=== Sorting and Limiting ===")
    print("    top_customers = nairobi_os.data.sql_query(")
    print("        handle_id,")
    print("        'SELECT customer_id, revenue FROM dataset ORDER BY revenue DESC LIMIT 10'")
    print("    )")
    print()

    print("=== Creating Derived Datasets ===")
    print("    high_value_handle = nairobi_os.data.sql_query(")
    print("        handle_id,")
    print("        'SELECT * FROM dataset WHERE transaction_amount > 5000'")
    print("    )")
    print()
    print("    # Then compute statistics on the derived dataset")
    print("    stats = json.loads(nairobi_os.data.crunch(high_value_handle, 'transaction_amount'))")
    print()
    print("    nairobi_os.data.free(high_value_handle)")
    print()

    print("=== Cleanup ===")
    print("    nairobi_os.data.free(handle_id)")
    print("    # nairobi_os.stop_refinery()  # Optional")

    print("\n✓ Refinery SQL queries demo completed!")

if __name__ == "__main__":
    main()