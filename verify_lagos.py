# File: verify_lagos.py
import nairobi_os
import time
from pathlib import Path

def test_pipeline():
    print("🚀 Igniting Data Plane...")
    nairobi_os.start_refinery()
    
    dataset = "simulator/PlayerStatisticsExtended.csv"
    if not Path(dataset).exists():
        print(f"❌ Dataset not found at {dataset}")
        return

    print("🚀 Ingesting Dataset...")
    raw_handle = nairobi_os.data.ingest(dataset)
    
    print("⚙️ Distilling 'points' column via Polars SQL...")
    # We need to make sure the result is binary f64 if we want zero-copy to work as expected
    # But for now, we test if the daemon spawns and parses the port.
    distilled_handle = nairobi_os.data.sql_query(raw_handle, "SELECT points FROM dataset")
    
    print("👁️ Spawning Lagos Vision (Headless Test)...")
    try:
        # This should spawn the daemon and return the widget
        widget = nairobi_os.lagos.plot_inline(distilled_handle, width=1000, height=400)
        print(f"✅ Lagos Widget created. Port: {widget.port}")
    except Exception as e:
        print(f"❌ Lagos spawn failed: {e}")
    finally:
        nairobi_os.stop_refinery()

if __name__ == "__main__":
    test_pipeline()
