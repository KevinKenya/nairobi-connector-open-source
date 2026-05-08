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
        
        # 1. Ingest (Using PyArrow engine for performance) - Timed
        ingest_start = time.perf_counter_ns()
        self.df = pd.read_csv(dataset_path, engine='pyarrow')
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
