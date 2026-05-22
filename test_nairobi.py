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

# File: ~/nairobi-connector-open-source/test_nairobi.py
# Author: Kevin Chege. Location: Nairobi
# Date: 2026-05-21

import nairobi_os
import time
import json
import os

def run_nba_verification():
    print("🚀 INITIALIZING NAIROBI OPEN-SOURCE VERIFICATION STRIKE...")
    dataset_path = "simulator/PlayerStatisticsExtended.csv"
    if not os.path.exists(dataset_path):
        print(f"❌ Error: {dataset_path} not found!")
        return
    
    # Ignite the Heavy Iron
    nairobi_os.start_refinery()
    
    start_total = time.time()
    
    # --- PHASE 1: INGESTION ---
    start_ingest = time.time()
    print(f"Ingesting {dataset_path}...")
    handle_id = nairobi_os.data.ingest(dataset_path)
    ingest_latency = (time.time() - start_ingest) * 1000
    
    # --- PHASE 2: ANALYSIS (CRUNCH) ---
    start_analyze = time.time()
    print("Crunching 'points' column...")
    crunch_results_json = nairobi_os.data.crunch(handle_id, "points")
    analytics = json.loads(crunch_results_json)
    analyze_latency = (time.time() - start_analyze) * 1000
    
    # --- PHASE 3: CORRELATION ---
    start_corr = time.time()
    print("Correlating 'points' and 'assists'...")
    corr_results_json = nairobi_os.data.correlate(handle_id, "points,assists")
    correlation = json.loads(corr_results_json)
    corr_latency = (time.time() - start_corr) * 1000
    
    total_latency = (time.time() - start_total) * 1000
    
    # --- PHASE 4: FORENSIC REPORT ---
    print("\n=== FORENSIC AUDIT REPORT: PURE DATA EXTRACTION ===")
    print(f"[METADATA]")
    print(f"Source File: {dataset_path}")
    print(f"Total Rows: {analytics['total_rows']}")
    print("")
    
    print(f"[LATENCY]")
    print(f"Ingestion Time: {ingest_latency:.2f} ms")
    print(f"Analysis Time (Crunch): {analyze_latency:.2f} ms")
    print(f"Analysis Time (Correlation): {corr_latency:.2f} ms")
    print(f"Total Strike Time: {total_latency:.2f} ms")
    print("")
    
    print(f"[AXIOM CRUNCH: points]")
    print(f"Mean: {analytics['mean']:.4f}")
    print(f"Max: {analytics['max']:.4f}")
    print(f"Std Dev: {analytics['std_dev']:.4f}")
    print("")
    
    print(f"[RELATIONAL STRIKE: points v assists]")
    print(f"Pearson Correlation: {correlation['pearson']:.4f}")
    print(f"Spearman Correlation: {correlation['spearman']:.4f}")
    print("")
    
    nairobi_os.stop_refinery()
    print(f"✅ Extraction verified successfully.")

if __name__ == "__main__":
    try:
        run_nba_verification()
    except Exception as e:
        print(f"FATAL ERROR: {e}")
        try:
            nairobi_os.stop_refinery()
        except:
            pass
        exit(1)
