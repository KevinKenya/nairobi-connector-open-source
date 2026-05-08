# Author: Kevin Chege. Location: Nairobi

# nairobi-benchmarks/engines/polars_engine.py
import polars as pl

class BenchmarkEngine:
    def __init__(self):
        self.config = None

    def setup(self, workload_config):
        self.config = workload_config

    def run(self):
        import time
        dataset_path = self.config['dataset']
        column = self.config.get('column', 'points')
        
        # 1. Ingest (Lazy Scan) - Timed
        ingest_start = time.perf_counter_ns()
        # Use infer_schema_length=None to scan entire file and correctly infer dtypes
        q = pl.scan_csv(dataset_path, infer_schema_length=None)
        # Force collection to measure ingestion time
        df = q.collect()
        ingest_ms = (time.perf_counter_ns() - ingest_start) / 1_000_000
        
        # 2. Build Query and Crunch - Timed
        crunch_start = time.perf_counter_ns()
        aggregations = [
            pl.col(column).mean().alias("mean"),
            pl.col(column).std().alias("std_dev"),
            pl.col(column).skew().alias("skewness"),
            pl.col(column).kurtosis().alias("kurtosis"),
        ]
        
        corr_ms = 0
        if 'corr_columns' in self.config:
            cols = self.config['corr_columns']
            aggregations.append(
                pl.corr(cols[0], cols[1], method="pearson").alias("pearson")
            )
        
        # Execute aggregations
        results = df.select(aggregations)
        crunch_ms = (time.perf_counter_ns() - crunch_start) / 1_000_000
        
        # 3. Correlation (if separate timing needed)
        # Note: Correlation is included in crunch timing above
        
        return {
            "ingest_ms": ingest_ms,
            "crunch_ms": crunch_ms,
            "corr_ms": corr_ms,
            "total_ms": ingest_ms + crunch_ms + corr_ms,
            "mean": float(results["mean"][0]),
            "std_dev": float(results["std_dev"][0]),
            "skewness": float(results["skewness"][0]),
            "kurtosis": float(results["kurtosis"][0]),
            "pearson": float(results["pearson"][0]) if 'pearson' in results.columns else None
        }

    def teardown(self):
        pass
