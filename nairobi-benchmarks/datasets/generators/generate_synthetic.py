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

# Author: Kevin Chege. Location: Nairobi

# nairobi-benchmarks/datasets/generators/generate_synthetic.py
import numpy as np
import pandas as pd
import pyarrow as pa
import pyarrow.parquet as pq
import argparse
import os
import time

def generate_tall_dataset(output_path, num_rows=10_000_000):
    """Generates a 10M row dataset with standard columns."""
    print(f"Generating Tall Dataset: {num_rows} rows...")
    t0 = time.time()
    
    np.random.seed(42)
    data = {
        'id': np.arange(num_rows, dtype=np.int64),
        'points': np.random.normal(20, 10, num_rows).astype(np.float64),
        'assists': np.random.normal(5, 3, num_rows).astype(np.float64),
        'rebounds': np.random.normal(8, 4, num_rows).astype(np.float64),
        'category': np.random.choice(['A', 'B', 'C', 'D'], num_rows)
    }
    
    df = pd.DataFrame(data)
    df.to_csv(output_path, index=False)
    
    t1 = time.time()
    print(f"Tall Dataset generated in {t1-t0:.2f}s at {output_path}")

def generate_wide_dataset(output_path, num_rows=100_000, num_cols=1000):
    """Generates a 1000-column dataset."""
    print(f"Generating Wide Dataset: {num_rows} rows x {num_cols} columns...")
    t0 = time.time()
    
    np.random.seed(42)
    data = {'id': np.arange(num_rows, dtype=np.int64)}
    
    for i in range(num_cols):
        data[f'col_{i:03d}'] = np.random.rand(num_rows).astype(np.float64)
        
    df = pd.DataFrame(data)
    df.to_csv(output_path, index=False)
    
    t1 = time.time()
    print(f"Wide Dataset generated in {t1-t0:.2f}s at {output_path}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate synthetic datasets for Nairobi benchmarks.")
    parser.add_argument("--type", choices=["tall", "wide"], required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--rows", type=int, default=10_000_000)
    parser.add_argument("--cols", type=int, default=1000)
    
    args = parser.parse_args()
    
    os.makedirs(os.path.dirname(os.path.abspath(args.output)), exist_ok=True)
    
    if args.type == "tall":
        generate_tall_dataset(args.output, args.rows)
    else:
        generate_wide_dataset(args.output, args.rows, args.cols)
