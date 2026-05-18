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

# nairobi-benchmarks/engines/pandas_engine.py
import pandas as pd
import numpy as np

class BenchmarkEngine:
    def __init__(self):
        self.df = None
        self.config = None

    def setup(self, workload_config):
        self.config = workload_config
        # We don't measure setup time in the 'run' loop
        pass

    def run(self):
        import time
        dataset_path = self.config['dataset']
        column = self.config.get('column', 'points')
        
        # 1. Ingest (Standard Junior implementation - no pyarrow optimization)
        ingest_start = time.perf_counter_ns()
        self.df = pd.read_csv(dataset_path)
        ingest_ms = (time.perf_counter_ns() - ingest_start) / 1_000_000
        
        # 2. Crunch (Statistical Distillation) - Timed
        # Using vectorized operations
        crunch_start = time.perf_counter_ns()
        mean = self.df[column].mean()
        std_dev = self.df[column].std()
        skewness = self.df[column].skew()
        kurtosis = self.df[column].kurt()
        crunch_ms = (time.perf_counter_ns() - crunch_start) / 1_000_000
        
        # 3. Correlation (if applicable) - Timed
        corr_ms = 0
        pearson = None
        if 'corr_columns' in self.config:
            cols = self.config['corr_columns']
            corr_start = time.perf_counter_ns()
            pearson = self.df[cols[0]].corr(self.df[cols[1]], method='pearson')
            corr_ms = (time.perf_counter_ns() - corr_start) / 1_000_000
            
        return {
            "ingest_ms": ingest_ms,
            "crunch_ms": crunch_ms,
            "corr_ms": corr_ms,
            "total_ms": ingest_ms + crunch_ms + corr_ms,
            "mean": float(mean),
            "std_dev": float(std_dev),
            "skewness": float(skewness),
            "kurtosis": float(kurtosis),
            "pearson": float(pearson) if pearson is not None else None
        }

    def teardown(self):
        self.df = None
