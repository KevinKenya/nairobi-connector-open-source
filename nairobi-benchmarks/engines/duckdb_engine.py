# Author: Kevin Chege. Location: Nairobi

# nairobi-benchmarks/engines/duckdb_engine.py
import duckdb

class BenchmarkEngine:
    def __init__(self):
        self.con = None
        self.config = None
        self.table_name = "benchmark_data"

    def setup(self, workload_config):
        self.config = workload_config
        self.con = duckdb.connect(database=':memory:')
    
    def run(self):
        import time
        dataset_path = self.config['dataset']
        column = self.config.get('column', 'points')
        
        # 1. Ingest - Create table from CSV (timed)
        ingest_start = time.perf_counter_ns()
        # Drop table if exists (for subsequent iterations)
        self.con.execute(f"DROP TABLE IF EXISTS {self.table_name}")
        # Create table - use sample_size=-1 to scan entire file for type detection
        # This will correctly identify numMinutes as VARCHAR due to values like "17:51"
        self.con.execute(f"""
            CREATE TABLE {self.table_name} AS 
            SELECT * FROM read_csv_auto('{dataset_path}', sample_size=-1)
        """)
        ingest_ms = (time.perf_counter_ns() - ingest_start) / 1_000_000
        
        # 2. Crunch - All statistics in one query (timed)
        crunch_start = time.perf_counter_ns()
        
        sql = f"""
        SELECT 
            AVG({column}) as mean, 
            STDDEV({column}) as std_dev, 
            SKEWNESS({column}) as skewness,
            KURTOSIS({column}) as kurtosis
        FROM {self.table_name}
        """
        
        res = self.con.execute(sql).fetchone()
        crunch_ms = (time.perf_counter_ns() - crunch_start) / 1_000_000
        
        # 3. Correlation (timed)
        corr_ms = 0
        pearson = None
        if 'corr_columns' in self.config:
            cols = self.config['corr_columns']
            corr_start = time.perf_counter_ns()
            # DuckDB CORR only supports 2 columns at a time
            pearson = self.con.execute(f"SELECT CORR({cols[0]}, {cols[1]}) FROM {self.table_name}").fetchone()[0]
            corr_ms = (time.perf_counter_ns() - corr_start) / 1_000_000
        
        return {
            "ingest_ms": ingest_ms,
            "crunch_ms": crunch_ms,
            "corr_ms": corr_ms,
            "total_ms": ingest_ms + crunch_ms + corr_ms,
            "mean": float(res[0]),
            "std_dev": float(res[1]),
            "skewness": float(res[2]),
            "kurtosis": float(res[3]) if res[3] is not None else 0.0,
            "pearson": float(pearson) if pearson is not None else None
        }
    
    def teardown(self):
        if self.con:
            self.con.close()
