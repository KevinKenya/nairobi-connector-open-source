# Copyright 2026 Kevin Chege
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# File: crates/nairobi-python/nairobi_os/__init__.py
# Author: Kevin Chege, Location: Nairobi, Date: 2026-05-21
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
Nairobi OS: High-Performance Data Science Infrastructure

This package provides Python bindings to the Nairobi OS Rust core, enabling
data scientists to leverage extreme performance through zero-copy data pipelines,
hardware acceleration, and kernel-bypass techniques.

Key Features:
- Zero-copy data ingestion using io_uring and huge pages
- Hardware-accelerated visualization via Lagos Vision
- Fused analytics pipeline for minimal latency
- Easy integration with Jupyter notebooks and pandas/numpy workflows
- Memory-efficient processing of large-scale datasets

Example Usage:
    >>> import nairobi_os
    >>> import pandas as pd
    >>> 
    >>> # Initialize the infrastructure (starts both Refinery and Hub daemons)
    >>> nairobi_os.ignite()
    >>> 
    >>> # Open the canvas to compile a DAG
    >>> dag_bytes = nairobi_os.canvas.open()
    >>> if dag_bytes:
    >>>     nairobi_os.canvas.execute(dag_bytes)
    >>> 
    >>> # Or use data pipeline directly
    >>> result = nairobi_os.data.pipeline(
    ...     "large_dataset.csv",
    ...     "value_column",
    ...     "col1,col2"
    ... )
    >>> 
    >>> # Convert results to pandas for further analysis
    >>> df_result = pd.json_normalize([result])
    >>> 
    >>> # Cleanup
    >>> nairobi_os.stop_refinery()
    """

__author__ = "Kevin Chege"
__version__ = "0.4.2"
__license__ = "Apache License 2.0"

# Import the _core module first to access its data functions
from . import _core

# Define a DataNamespace class to support nairobi_os.data.ingest pattern
class DataNamespace:
    def __init__(self):
        self.ingest = _core.data.ingest
        self.sql_query = _core.data.sql_query
        self.crunch = _core.data.crunch
        self.correlate = _core.data.correlate
        self.pipeline = _core.data.pipeline
        self.crunch_and_correlate = _core.data.crunch_and_correlate
        self.free = _core.data.free
        self.get_fd = _core.data.get_fd

data = DataNamespace()

# Expose data functions from _core.data at module level
ingest = _core.data.ingest
sql_query = _core.data.sql_query
crunch = _core.data.crunch
correlate = _core.data.correlate
pipeline = _core.data.pipeline
crunch_and_correlate = _core.data.crunch_and_correlate
free = _core.data.free
get_fd = _core.data.get_fd

# Canvas namespace for nairobi_os.canvas.* pattern
class CanvasNamespace:
    def __init__(self):
        self.open = _core.canvas.open
        self.execute = _core.canvas.execute
        self.build_dag = _core.canvas.build_dag

canvas = CanvasNamespace()

# Import framework, lagos, and ui modules
from . import framework
from . import lagos
from .ui import ui

# Export specific public functions from framework that we want at module level
from .framework import SovereignFrame, ColumnAccessor

# Process management functions for starting/stopping the refinery daemon
import os
import time
import subprocess
import logging
from pathlib import Path

# Set up logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Global reference to the refinery process
_refinery_process = None
_hub_process = None

def start_refinery(binary_path=None, timeout=15):
    """
    Start the Nairobi Axum Refinery daemon.
    """
    global _refinery_process
    
    if binary_path is None:
        bin_dir = Path(__file__).parent / "bin"
        binary_path = bin_dir / "nairobi-axum-refinery"
    
    binary_path = Path(binary_path)
    if not binary_path.exists():
        raise RuntimeError(f"Refinery binary not found at {binary_path}")
    
    if _refinery_process is not None:
        if _refinery_process.poll() is None:
            return True
        else:
            _refinery_process = None
    
    try:
        # Forensic Log Path
        log_path = Path.home() / ".nairobi_refinery.log"
        log_file = open(log_path, "a")
        
        # Launch as a Sovereign Daemon
        _refinery_process = subprocess.Popen(
            [str(binary_path)],
            start_new_session=True,
            stdout=log_file,
            stderr=log_file
        )
        log_file.close()  # Child inherited the fd; parent must close its copy
        
        logger.info(f"🚀 Igniting Axum Refinery (PID: {_refinery_process.pid})")
        logger.info(f"📝 Logs: {log_path}")
        
        # Wait for D-Bus registration
        start_time = time.time()
        while time.time() - start_time < timeout:
            if _check_dbus_service():
                logger.info("✅ Axum Refinery is live on D-Bus")
                return True
            
            # Check if the process died immediately
            if _refinery_process.poll() is not None:
                break
                
            time.sleep(0.5)
        
        # Failure handling
        if _refinery_process.poll() is not None:
            error_msg = "Refinery process exited immediately. Check ~/.nairobi_refinery.log"
        else:
            error_msg = f"Systemic Seizure: Refinery failed to register on D-Bus within {timeout}s"
            stop_refinery()
            
        raise RuntimeError(error_msg)
        
    except Exception as e:
        raise RuntimeError(f"Failed to ignite refinery: {e}")

def _check_dbus_service():
    """
    Forensic check using the CORRECT service name from the Nairobi Protocol.
    """
    try:
        # Corrected Name: org.nairobi.NairobiAxumRefinery1
        result = subprocess.run(
            ["busctl", "--user", "status", "org.nairobi.NairobiAxumRefinery1"],
            capture_output=True,
            text=True
        )
        return result.returncode == 0
    except Exception:
        return False

def stop_refinery():
    """Stop the Refinery daemon and its spawned Hub daemon."""
    global _refinery_process, _hub_process
    if _refinery_process:
        try:
            _refinery_process.terminate()
            _refinery_process.wait(timeout=2)
        except Exception:
            _refinery_process.kill()
        _refinery_process = None
        logger.info("🛑 Refinery stopped.")
    if _hub_process:
        try:
            _hub_process.terminate()
            _hub_process.wait(timeout=2)
        except Exception:
            _hub_process.kill()
        _hub_process = None
        logger.info("🛑 Hub stopped.")

def ignite(binary_path=None, timeout=15):
    """
    Start both Nairobi Axum Refinery and Nairobi Hub daemons.
    This is the primary entry point for initializing the Nairobi OS infrastructure.
    """
    global _refinery_process, _hub_process
    
    # Start Refinery first (Hub depends on it)
    start_refinery(binary_path, timeout)
    
    # Start Hub daemon
    if _hub_process is not None:
        if _hub_process.poll() is None:
            return True
        else:
            _hub_process = None
    
    bin_dir = Path(__file__).parent / "bin"
    hub_binary = bin_dir / "nairobi-hub"
    
    if not hub_binary.exists():
        raise RuntimeError(f"Hub binary not found at {hub_binary}")
    
    try:
        log_path = Path.home() / ".nairobi_hub.log"
        log_file = open(log_path, "a")
        
        # Prepare environment with LAGOS_VISION_DAEMON_BIN
        env = os.environ.copy()
        if "LAGOS_VISION_DAEMON_BIN" not in env:
            env["LAGOS_VISION_DAEMON_BIN"] = str(bin_dir / "lagos-vision-daemon")
        
        _hub_process = subprocess.Popen(
            [str(hub_binary)],
            start_new_session=True,
            stdout=log_file,
            stderr=log_file,
            env=env
        )
        log_file.close()  # Child inherited the fd; parent must close its copy
        
        logger.info(f"🚀 Igniting Nairobi Hub (PID: {_hub_process.pid})")
        logger.info(f"📝 Logs: {log_path}")
        
        start_time = time.time()
        while time.time() - start_time < timeout:
            if _check_hub_service():
                logger.info("✅ Nairobi Hub is live on D-Bus")
                return True
            
            if _hub_process.poll() is not None:
                break
                
            time.sleep(0.5)
        
        if _hub_process.poll() is not None:
            error_msg = "Hub process exited immediately. Check ~/.nairobi_hub.log"
        else:
            error_msg = f"Systemic Seizure: Hub failed to register on D-Bus within {timeout}s"
            if _hub_process is not None:
                _hub_process.terminate()
            _hub_process = None
            
        raise RuntimeError(error_msg)
        
    except Exception as e:
        raise RuntimeError(f"Failed to ignite hub: {e}")

def _check_hub_service():
    """Check if Hub service is registered on D-Bus."""
    try:
        result = subprocess.run(
            ["busctl", "--user", "status", "org.nairobi.NairobiHub1"],
            capture_output=True,
            text=True
        )
        return result.returncode == 0
    except Exception:
        return False