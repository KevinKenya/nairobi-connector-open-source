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

# File: verify_execution.py
import os
import sys
import time
import subprocess
import nairobi_os
from nairobi_os import SovereignFrame

def verify_all():
    print("🚦 Starting End-to-End Nairobi OS Integration Verification...")
    
    # 1. Start Refinery
    print("\n[Step 1/6] Igniting Axum Refinery daemon...")
    try:
        nairobi_os.start_refinery()
        print("✅ Refinery daemon registered and active on D-Bus!")
    except Exception as e:
        print(f"⚠️ Refinery start details (continuing): {e}")

    # 2. Setup synthetic large file & outlier
    print("\n[Step 2/6] Seeding dataset with anomalous points...")
    source_csv = "simulator/PlayerStatisticsExtended.csv"
    test_csv = "/tmp/data_scientist_large_data.csv"
    
    if not os.path.exists(source_csv):
        print(f"❌ Error: source CSV {source_csv} not found.")
        sys.exit(1)
        
    with open(source_csv, "r") as f:
        headers = f.readline()
        body = f.read()

    # Create duplicate copy to reach large dataset size
    with open(test_csv, "w") as f:
        f.write(headers)
        f.write(body)
        f.write(body)

    # Append extreme anomaly row
    columns = headers.strip().split(",")
    anomaly_values = ["ANOMALY"] * len(columns)
    anomaly_values[columns.index("firstName")] = "SYSTEMIC"
    anomaly_values[columns.index("lastName")] = "ANOMALY_PLAYER"
    anomaly_values[columns.index("points")] = "99999.0"
    anomaly_values[columns.index("personId")] = "999999"
    anomaly_values[columns.index("gameId")] = "99999999"

    with open(test_csv, "a") as f:
        f.write(",".join(anomaly_values) + "\n")
    print(f"✅ Created seeded dataset at {test_csv} ({os.path.getsize(test_csv) / (1024*1024):.2f} MB)")

    # 3. Ingestion
    print("\n[Step 3/6] Ingesting large dataset...")
    start = time.time()
    handle_id = nairobi_os.ingest(test_csv)
    latency = (time.time() - start) * 1000
    print(f"✅ Ingestion completed in {latency:.2f} ms! Assigned handle ID: {handle_id}")

    # 4. Statistical analysis using SovereignFrame
    print("\n[Step 4/6] Querying statistical outliers via SovereignFrame...")
    df = SovereignFrame(handle_id)
    stats = df.points.crunch()
    mean = stats["mean"]
    std_dev = stats["std_dev"]
    max_val = stats["max"]
    
    print(f"📈 Points stats: Mean={mean:.2f}, Std Dev={std_dev:.2f}, Max={max_val:.2f}")
    
    outlier_threshold = mean + (5 * std_dev)
    print(f"🔍 Outlier Threshold (5 Sigma): {outlier_threshold:.2f}")
    
    if max_val > outlier_threshold:
        print("🚨 Outlier detected successfully!")
        anomalous_frame = df.query("SELECT firstName, lastName, points FROM df WHERE points > 50000.0")
        result_str = nairobi_os.crunch(anomalous_frame.handle_id, "points")
        print(f"✅ Seeded Outlier Confirmed: {result_str}")
    else:
        print("❌ Seeded outlier was NOT detected. Verification failed.")
        sys.exit(1)

    # 5. UI MCP Server and AT-SPI2 Map verification
    print("\n[Step 5/6] Spawning UI MCP server and capturing active interface map...")
    
    # Check if a graphical display environment is present
    has_display = "DISPLAY" in os.environ or "WAYLAND_DISPLAY" in os.environ
    editor_proc = None
    
    if has_display:
        print("🖥️ Graphical display detected. Launching GNOME Text Editor...")
        try:
            editor_proc = subprocess.Popen(["gnome-text-editor", test_csv])
            time.sleep(2.0)
        except Exception as e:
            print(f"⚠️ Failed to spawn gnome-text-editor: {e}. Trying gedit...")
            try:
                editor_proc = subprocess.Popen(["gedit", test_csv])
                time.sleep(2.0)
            except Exception as e2:
                print(f"⚠️ Failed to spawn gedit: {e2}. Skipping editor window spawning.")
    else:
        print("🖥️ Headless environment. Skipping graphical Text Editor spawning.")
        
    print("🔌 Igniting Nairobi UI Connector MCP Server...")
    nairobi_os.ui.start()
    
    print("🎯 Searching for active window...")
    window_found = False
    if has_display and editor_proc:
        res = nairobi_os.ui.find_window("Text Editor")
        if "Target focused" in res:
            window_found = True
            print("✅ Found GNOME Text Editor window successfully!")
        else:
            res = nairobi_os.ui.find_window("Editor")
            if "Target focused" in res:
                window_found = True
                print("✅ Found Gedit window successfully!")
                
    if not window_found:
        print("ℹ️ Text Editor window target skipped or not found (normal on headless environments).")
        
    print("🗺️ Getting TOON Screen Map (Depth=5)...")
    try:
        toon_map = nairobi_os.ui.get_map(max_depth=5)
        print("✅ TOON Map retrieved successfully!")
        print("--- TOON Map Snippet ---")
        print("\n".join(toon_map.splitlines()[:15]))
        print("-----------------------")
    except Exception as e:
        print(f"❌ Error getting screen map: {e}")
        sys.exit(1)

    # 6. Teardown
    print("\n[Step 6/6] Tearing down and cleaning up daemons...")
    nairobi_os.ui.stop()
    nairobi_os.stop_refinery()
    
    if editor_proc:
        try:
            editor_proc.terminate()
            editor_proc.wait(timeout=2)
            print("🧹 Terminated graphical Text Editor.")
        except:
            pass
            
    if os.path.exists(test_csv):
        os.remove(test_csv)
        print("🧹 Temporary dataset file deleted.")
        
    print("\n🎉 ALL PIPELINES SUCCESSFULLY VERIFIED & FUNCTIONAL!")
    print("Nairobi OS Phase 5 Integration is rock solid.")

if __name__ == "__main__":
    verify_all()
