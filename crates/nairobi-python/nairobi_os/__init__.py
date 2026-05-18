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
    >>> # Start the refinery daemon
    >>> nairobi_os.start_refinery()
    >>> 
    >>> # Ingest and analyze data in a single call
    >>> result = nairobi_os.data.pipeline(
    ...     "large_dataset.csv",
    ...     "value_column",
    ...     "col1,col2"
    ... )
    >>> 
    >>> # Convert results to pandas for further analysis
    >>> df_result = pd.json_normalize([result])
    >>> 
    >>> # Create interactive visualization
    >>> widget = nairobi_os.lagos.plot_inline(handle_id="abc-123")
    >>> widget  # Display in Jupyter
    >>> 
    >>> # Cleanup
    >>> nairobi_os.stop_refinery()
    """

__author__ = "Kevin Chege"
__version__ = "0.3.1"
__license__ = "Apache License 2.0"

# Import the _core module first to access its data functions
from . import _core

# Expose data functions from _core.data at module level
ingest = _core.data.ingest
sql_query = _core.data.sql_query
crunch = _core.data.crunch
correlate = _core.data.correlate
pipeline = _core.data.pipeline
crunch_and_correlate = _core.data.crunch_and_correlate
free = _core.data.free
get_fd = _core.data.get_fd

# Import framework and lagos modules
from . import framework
from . import lagos

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
    global _refinery_process
    if _refinery_process:
        try:
            _refinery_process.terminate()
            _refinery_process.wait(timeout=2)
        except Exception:
            _refinery_process.kill()
        _refinery_process = None
        logger.info("🛑 Refinery stopped.")

#