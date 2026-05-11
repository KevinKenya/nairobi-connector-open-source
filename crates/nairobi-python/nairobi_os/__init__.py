# File: /home/chege/nairobi-connector-open-source/crates/nairobi-python/nairobi_os/__init__.py
# Author: Kevin Chege. Location: Nairobi
# Date: 2026-05-06

# nairobi-open-source-release/crates/nairobi-python/nairobi_os/__init__.py
"""
Nairobi OS Python Bindings - Core Wrapper Module
"""

import os
import time
import subprocess
import logging
from pathlib import Path

# 1. Import the compiled Rust binary
from . import _core

# 2. Extract submodules from the binary into the nairobi_os namespace
from . import lagos
data = _core.data

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

__all__ = ["data", "lagos", "start_refinery", "stop_refinery"]
