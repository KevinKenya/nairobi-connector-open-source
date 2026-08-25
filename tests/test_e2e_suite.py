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

# File: tests/test_e2e_suite.py
# Author: Kevin Chege, Location: Nairobi, Date: 25th August 2026

import os
import sys
import time
import json
import pytest
import pandas as pd
import numpy as np

import nairobi_os
from nairobi_os import SovereignFrame

TEST_CSV = "/tmp/nairobi_e2e_test_data.csv"

@pytest.fixture(scope="module", autouse=True)
def setup_test_data():
    """Generates synthetic dataset for E2E testing."""
    np.random.seed(42)
    df = pd.DataFrame({
        "personId": range(1, 201),
        "firstName": [f"Player_{i}" for i in range(1, 201)],
        "lastName": [f"LastName_{i}" for i in range(1, 201)],
        "points": np.random.uniform(5.0, 35.0, 200),
        "assists": np.random.uniform(0.0, 12.0, 200),
        "rebounds": np.random.uniform(1.0, 15.0, 200),
    })
    # Add outlier
    df.loc[0, "points"] = 999.0
    df.to_csv(TEST_CSV, index=False)
    yield TEST_CSV
    if os.path.exists(TEST_CSV):
        os.remove(TEST_CSV)

@pytest.fixture(scope="module", autouse=True)
def start_stop_services():
    """Ensures Refinery daemon is running during tests."""
    nairobi_os.start_refinery()
    yield
    try:
        nairobi_os.stop_refinery()
    except Exception:
        pass

def test_01_refinery_ingestion_and_crunch():
    """Tests zero-copy CSV ingestion and statistical crunching."""
    handle_id = nairobi_os.data.ingest(TEST_CSV)
    assert handle_id is not None and len(handle_id) > 0

    crunch_json = nairobi_os.data.crunch(handle_id, "points")
    res = json.loads(crunch_json)

    assert res["total_rows"] == 200
    assert res["max"] >= 999.0
    assert "mean" in res
    assert "std_dev" in res

def test_02_refinery_correlation():
    """Tests multi-column statistical correlation."""
    handle_id = nairobi_os.data.ingest(TEST_CSV)
    corr_json = nairobi_os.data.correlate(handle_id, "points,assists")
    res = json.loads(corr_json)

    assert "pearson" in res
    assert "spearman" in res

def test_03_sovereign_frame_api():
    """Tests SovereignFrame high-level Python API and SQL querying."""
    handle_id = nairobi_os.data.ingest(TEST_CSV)
    df = SovereignFrame(handle_id)
    assert df.handle_id is not None

    stats = df.points.crunch()
    assert stats["total_rows"] == 200
    assert stats["max"] >= 999.0

    filtered_df = df.query("SELECT firstName, lastName, points FROM df WHERE points > 500.0")
    assert isinstance(filtered_df, SovereignFrame)
    filtered_stats = filtered_df.points.crunch()
    assert filtered_stats["total_rows"] == 1
    assert filtered_stats["max"] == 999.0

def test_04_canvas_bridge_gvariant():
    """Tests GVariant DAG building via Canvas module."""
    nodes = [
        (1, "Ingest", json.dumps({"dataset_path": TEST_CSV})),
        (2, "AxiomCrunch", json.dumps({"column": "points", "mean": True, "std_dev": True, "kurtosis": False}))
    ]
    edges = [
        (1, 2)
    ]
    dag_bytes = nairobi_os.canvas.build_dag(nodes, edges)
    assert isinstance(dag_bytes, (bytes, bytearray, list))
    assert len(dag_bytes) > 0

def test_05_ui_mcp_connector_lifecycle():
    """Tests UI MCP connector start, map acquisition, and stop."""
    nairobi_os.ui.start()
    try:
        toon_map = nairobi_os.ui.get_map(max_depth=2)
        assert isinstance(toon_map, str)
    finally:
        nairobi_os.ui.stop()

if __name__ == "__main__":
    pytest.main(["-v", __file__])
