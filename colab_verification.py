# %% [markdown]
# ### **1. THE COLAB BOOTSTRAP (ENVIRONMENT PREP)**

# %%
# Install D-Bus and kagglehub
!apt-get update && apt-get install -y dbus-x11 -qq
!pip install kagglehub nairobi_os -q

import os
import subprocess
import kagglehub
import nairobi_os

# 🔌 Initialize D-Bus Session
print("🔌 Initializing D-Bus Session...")
try:
    dbus_out = subprocess.check_output(["dbus-launch"]).decode()
    for line in dbus_out.splitlines():
        if "=" in line:
            k, v = line.split("=", 1)
            os.environ[k] = v.replace(";", "").replace("'", "").replace('"', '')
    print("✅ D-Bus session initialized.")
except Exception as e:
    print(f"❌ Failed to initialize D-Bus: {e}")

# 📂 Download the real NBA data using kagglehub
print("📂 Downloading dataset...")
dataset_path = kagglehub.dataset_download('eoinamoore/historical-nba-data-and-player-box-scores')
csv_path = os.path.join(dataset_path, "PlayerStatisticsExtended.csv")
print(f'✅ Data source import complete: {csv_path}')

# %% [markdown]
# ### **2. THE LIVE-FIRE CELL**

# %%
# 🔥 Ignite the Heavy Iron (Nairobi OS)
print("🔥 Igniting the Axum Refinery...")
try:
    nairobi_os.start_refinery()
    print("✅ AXUM REFINERY ONLINE")
except Exception as e:
    print(f"💥 Ignition Failed: {e}")
    # Fallback to check logs
    !cat ~/.nairobi_refinery.log 2>/dev/null || echo 'No log file'

# 🚀 Execute Data Pipeline
print("🚀 Ingesting NBA dataset...")
handle = nairobi_os.data.ingest(csv_path)

print("🔍 Filtering for 'points' (Points) column...")
points_handle = nairobi_os.data.sql_query(handle, "SELECT points FROM dataset")

print("👁️ Launching Lagos Visual Cortex...")
nairobi_os.lagos.plot_inline(points_handle)
