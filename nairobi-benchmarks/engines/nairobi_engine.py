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

# nairobi-benchmarks/engines/nairobi_engine.py
# Updated to use fused pipeline() for maximum performance
import nairobi_os
import json

class BenchmarkEngine:
    def __init__(self):
        self.config = None

    def setup(self, workload_config):
        self.config = workload_config
        # Ignite once per session (as per Caesar's rationale)
        nairobi_os.start_refinery()

    def run(self):
        import time
        dataset_path = self.config['dataset']
        column = self.config.get('column', 'points')
        has_corr = 'corr_columns' in self.config and len(self.config['corr_columns']) >= 2
        
        # 1. Ingestion Stage
        ingest_start = time.perf_counter_ns()
        handle = nairobi_os.data.ingest(dataset_path)
        ingest_ms = (time.perf_counter_ns() - ingest_start) / 1_000_000
        
        # 2. Crunch Stage
        crunch_start = time.perf_counter_ns()
        crunch_json = nairobi_os.data.crunch(handle, column)
        crunch_ms = (time.perf_counter_ns() - crunch_start) / 1_000_000
        crunch_res = json.loads(crunch_json)
        
        # 3. Correlation Stage (if applicable)
        corr_ms = 0
        pearson = None
        if has_corr:
            cols = self.config['corr_columns']
            corr_str = f"{cols[0]},{cols[1]}"
            corr_start = time.perf_counter_ns()
            corr_json = nairobi_os.data.correlate(handle, corr_str)
            corr_ms = (time.perf_counter_ns() - corr_start) / 1_000_000
            corr_res = json.loads(corr_json)
            pearson = corr_res.get('pearson')

        return {
            "ingest_ms": ingest_ms,
            "crunch_ms": crunch_ms,
            "corr_ms": corr_ms,
            "total_ms": ingest_ms + crunch_ms + corr_ms,
            "mean": float(crunch_res['mean']),
            "std_dev": float(crunch_res['std_dev']),
            "skewness": float(crunch_res['skewness']),
            "kurtosis": float(crunch_res['kurtosis']),
            "pearson": float(pearson) if pearson is not None else None
        }

    def teardown(self):
        nairobi_os.stop_refinery()
