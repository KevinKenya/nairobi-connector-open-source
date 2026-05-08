# Author: Kevin Chege. Location: Nairobi

import os
import yaml
import time
import json
import csv
import argparse
import importlib
import sys
import subprocess
from datetime import datetime
from metrics_collector import MetricsCollector, measure_wall_time_ns
from result_validator import ResultValidator

def drop_caches(force=False):
    if not force:
        print("[INFO] Hot cache mode - skipping cache drop.")
        return
    print("Attempting to drop caches (Cold Cache Mandate)...")
    try:
        # Use subprocess to properly handle sudo
        result = subprocess.run(
            ["sudo", "sh", "-c", "sync && echo 3 > /proc/sys/vm/drop_caches"],
            capture_output=True,
            text=True,
            timeout=10
        )
        if result.returncode != 0:
            raise PermissionError(f"Sudo failed: {result.stderr}")
        print("[SUCCESS] Caches dropped successfully.")
    except subprocess.TimeoutExpired:
        print("\n[WARNING]: Sudo timeout. Results represent WARM cache only.")
    except Exception as e:
        print(f"\n[WARNING]: Cold cache drop failed. Results represent WARM cache only. Error: {e}")

def load_engine(engine_name):
    module_name = f"engines.{engine_name}_engine"
    try:
        sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
        module = importlib.import_module(module_name)
        return module.BenchmarkEngine()
    except Exception as e:
        print(f"Error loading engine {engine_name}: {e}")
        return None

def run_benchmark(workload_file, engines_to_run, iterations=10, cold_cache=True):
    with open(workload_file, 'r') as f:
        workload = yaml.safe_load(f)

    print(f"=== Starting Benchmark: {workload['name']} ===")
    
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    results_raw = {
        "workload": workload['name'],
        "timestamp": timestamp,
        "iterations": iterations,
        "runs": []
    }

    validator = ResultValidator()
    reference_results = None
    
    for engine_name in engines_to_run:
        engine = load_engine(engine_name)
        if not engine:
            continue

        print(f"\n🚀 Engine: {engine_name.upper()}")
        
        # Cold Cache Mandate
        drop_caches(cold_cache)

        engine_runs = []
        
        engine.setup(workload)

        for i in range(iterations):
            print(f"  Iteration {i+1}/{iterations}...", end="\r")
            
            collector = MetricsCollector()
            collector.start()
            
            try:
                math_results, wall_time_ns = measure_wall_time_ns(engine.run)
                collector.stop()
                metrics = collector.get_metrics()
                
                valid = True
                if i == 0:
                    valid, msg = validator.validate(engine_name, math_results, reference_results)
                    print(f"\n  {msg}")
                    if not valid and reference_results is not None:
                        print(f"  [DISQUALIFIED] Mathematical inaccuracy detected.")
                    if reference_results is None and valid:
                        reference_results = math_results

                engine_runs.append({
                    "iteration": i,
                    "latency_ms": wall_time_ns / 1_000_000,
                    "ingest_ms": math_results.get("ingest_ms", 0),
                    "crunch_ms": math_results.get("crunch_ms", 0),
                    "corr_ms": math_results.get("corr_ms", 0),
                    "total_ms": math_results.get("total_ms", wall_time_ns / 1_000_000),
                    "peak_cpu_percent": metrics['peak_cpu_percent'],
                    "peak_ram_mb": metrics['peak_ram_mb'],
                    "valid": valid
                })
            except Exception as e:
                collector.stop()
                print(f"\n  [ERROR] Iteration {i} failed: {e}")
                break

        engine.teardown()
        
        results_raw["runs"].append({
            "engine": engine_name,
            "data": engine_runs
        })

    save_results(results_raw, timestamp)

def save_results(raw_data, timestamp):
    os.makedirs("results/raw", exist_ok=True)
    os.makedirs("results/processed", exist_ok=True)
    
    raw_path = f"results/raw/run_{timestamp}.json"
    with open(raw_path, 'w') as f:
        json.dump(raw_data, f, indent=2)
    print(f"\nRaw telemetry saved to {raw_path}")

    summary_path = "results/processed/summary.csv"
    file_exists = os.path.isfile(summary_path)
    
    with open(summary_path, 'a', newline='') as f:
        writer = csv.writer(f)
        if not file_exists:
            writer.writerow(["Timestamp", "Workload", "Engine", "Mean Latency (ms)", "StdDev Latency (ms)", "Peak RAM (MB)", "Peak CPU (%)"])
        
        for run in raw_data['runs']:
            engine = run['engine']
            latencies = [d['latency_ms'] for d in run['data']]
            if not latencies: continue
            
            mean_lat = sum(latencies) / len(latencies)
            std_dev_lat = (sum([(x - mean_lat)**2 for x in latencies]) / len(latencies))**0.5
            peak_ram = max([d['peak_ram_mb'] for d in run['data']])
            peak_cpu = max([d['peak_cpu_percent'] for d in run['data']])
            
            writer.writerow([timestamp, raw_data['workload'], engine, f"{mean_lat:.2f}", f"{std_dev_lat:.2f}", f"{peak_ram:.2f}", f"{peak_cpu:.2f}"])

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--workload", required=True)
    parser.add_argument("--engines", nargs="+", default=["pandas", "polars", "duckdb", "nairobi"])
    parser.add_argument("--iterations", type=int, default=10)
    parser.add_argument("--cold-cache", action="store_true", help="Enable cold cache (drops system caches before each engine)")
    args = parser.parse_args()
    
    run_benchmark(args.workload, args.engines, args.iterations, args.cold_cache)
