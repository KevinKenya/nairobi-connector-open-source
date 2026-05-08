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
        
        # Check if we have correlation columns for the fused pipeline
        has_corr = 'corr_columns' in self.config and len(self.config['corr_columns']) >= 2
        
        if has_corr:
            # FUSED PIPELINE: Single D-Bus round trip for ingest + crunch + correlate
            cols = self.config['corr_columns']
            corr_str = f"{cols[0]},{cols[1]}"
            
            pipeline_start = time.perf_counter_ns()
            result_json = nairobi_os.data.pipeline(dataset_path, column, corr_str)
            pipeline_ms = (time.perf_counter_ns() - pipeline_start) / 1_000_000
            result = json.loads(result_json)
            
            return {
                "ingest_ms": 0,  # Fused - can't separate
                "crunch_ms": 0,  # Fused - can't separate
                "corr_ms": 0,    # Fused - can't separate
                "total_ms": pipeline_ms,
                "mean": float(result['mean']),
                "std_dev": float(result['std_dev']),
                "skewness": float(result['skewness']),
                "kurtosis": float(result['kurtosis']),
                "pearson": float(result['pearson'])
            }
        else:
            # STANDARD PATH: Ingest + Crunch (no correlation)
            ingest_start = time.perf_counter_ns()
            handle = nairobi_os.data.ingest(dataset_path)
            ingest_ms = (time.perf_counter_ns() - ingest_start) / 1_000_000
            
            crunch_start = time.perf_counter_ns()
            crunch_json = nairobi_os.data.crunch(handle, column)
            crunch_ms = (time.perf_counter_ns() - crunch_start) / 1_000_000
            crunch_res = json.loads(crunch_json)
            
            return {
                "ingest_ms": ingest_ms,
                "crunch_ms": crunch_ms,
                "corr_ms": 0,
                "total_ms": ingest_ms + crunch_ms,
                "mean": float(crunch_res['mean']),
                "std_dev": float(crunch_res['std_dev']),
                "skewness": float(crunch_res['skewness']),
                "kurtosis": float(crunch_res['kurtosis']),
                "pearson": None
            }

    def teardown(self):
        nairobi_os.stop_refinery()
