# File: /home/KevinKenya/nairobi-connector-open-source/crates/nairobi-python/nairobi_os/__init__.py
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
from .framework import SovereignFrame

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
        # Fallback for development mode
        dev_path = Path(__file__).parents[3] / "target" / "release" / "nairobi-axum-refinery"
        if dev_path.exists():
            binary_path = dev_path
        else:
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
        if result.returncode == 0:
            return True

        # Alternative check using dbus-send if busctl is picky or unavailable
        result = subprocess.run(
            ["dbus-send", "--session", "--dest=org.freedesktop.DBus", "--type=method_call", "--print-reply", "/org/freedesktop/DBus", "org.freedesktop.DBus.ListNames"],
            capture_output=True,
            text=True
        )
        return "org.nairobi.NairobiAxumRefinery1" in result.stdout
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

def connect():
    """
    Auto-configures XDG_RUNTIME_DIR, starts D-Bus (if needed), and ignites the background daemons.
    Includes Colab environment armor for headless operation.
    """
    # 1. Environment Setup: Ensure XDG_RUNTIME_DIR exists
    if 'XDG_RUNTIME_DIR' not in os.environ:
        runtime_dir = Path("/tmp/runtime-root")
        try:
            runtime_dir.mkdir(mode=0o700, exist_ok=True)
            os.environ['XDG_RUNTIME_DIR'] = str(runtime_dir)
            logger.info(f"🔧 Created XDG_RUNTIME_DIR: {runtime_dir}")
        except Exception as e:
            logger.warning(f"⚠️ Failed to create XDG_RUNTIME_DIR: {e}")

    # 2. D-Bus Auto-Ignition for Headless Environments (Colab, SSH, etc.)
    # We check multiple environment variables and try to connect to the session bus.
    dbus_ready = False
    if 'DBUS_SESSION_BUS_ADDRESS' in os.environ:
        try:
            # Test if existing address works
            subprocess.run(["busctl", "--user", "status", "org.freedesktop.DBus"],
                           capture_output=True, timeout=2, check=True)
            dbus_ready = True
        except Exception:
            logger.info("🔗 DBUS_SESSION_BUS_ADDRESS is set but connection failed. Re-initializing...")

    if not dbus_ready:
        try:
            # Use dbus-launch to spawn a new session and capture its variables
            output = subprocess.check_output(["dbus-launch"], text=True)
            for line in output.splitlines():
                if '=' in line:
                    key, val = line.split('=', 1)
                    # Remove trailing semicolon and quotes
                    val = val.strip().rstrip(';').strip("'").strip('"')
                    os.environ[key] = val
                    logger.debug(f"DBUS ENV: {key}={val}")
            
            if 'DBUS_SESSION_BUS_ADDRESS' in os.environ:
                logger.info(f"✅ D-Bus Session Started: {os.environ['DBUS_SESSION_BUS_ADDRESS'][:40]}...")
                dbus_ready = True
            else:
                logger.error("❌ dbus-launch failed to provide DBUS_SESSION_BUS_ADDRESS")
        except Exception as e:
            logger.error(f"💥 Failed to launch D-Bus session: {e}")

    # 3. Refinery Ignition
    return start_refinery()

def read_csv(path, delimiter=",", encoding="utf-8"):
    """
    Ingests data via Axum and returns a SovereignFrame object.
    Automatically ignites the refinery (if possible) and implements Lazy Ignition.
    """
    try:
        handle_id = data.ingest(str(path), delimiter, encoding)
        return SovereignFrame(handle_id)
    except Exception as e:
        logger.info("⚠️ Refinery offline or connection failed. Attempting Lazy Ignition...")
        try:
            start_refinery()
            time.sleep(2) # Wait for bus registration
            handle_id = data.ingest(str(path), delimiter, encoding)
            return SovereignFrame(handle_id)
        except Exception as retry_err:
            raise RuntimeError(f"Failed to ingest CSV after Lazy Ignition: {retry_err}") from e

__all__ = ["data", "lagos", "start_refinery", "stop_refinery", "connect", "read_csv", "SovereignFrame"]
