# File: /home/chege/nairobi-connector-open-source/crates/nairobi-python/nairobi_os/lagos.py
# Author: Kevin Chege. Location: Nairobi
# Date: 2026-05-11

import anywidget
import traitlets
import subprocess
import os
import re
import logging
from pathlib import Path

logger = logging.getLogger(__name__)

class LagosWidget(anywidget.AnyWidget):
    """
    Lagos Vision Inline Widget for Jupyter Lab.
    Streams JPEG frames from the lagos-vision-daemon via WebSocket.
    """
    _esm = """
    export function render({ model, el }) {
      const canvas = document.createElement("canvas");
      canvas.style.width = "100%";
      canvas.style.height = "auto";
      canvas.width = model.get("width");
      canvas.height = model.get("height");
      el.appendChild(canvas);
      
      const ctx = canvas.getContext("2d");
      const port = model.get("port");
      
      // Draw loading state
      ctx.fillStyle = "#111";
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      ctx.fillStyle = "#fff";
      ctx.font = "20px Inter, system-ui";
      ctx.fillText("👁️ Connecting to Lagos Visual Cortex...", 20, 40);

      if (!port) return;

      const ws = new WebSocket(`ws://127.0.0.1:${port}`);
      ws.binaryType = "blob";

      ws.onopen = () => {
        console.log(`[LAGOS] Connected to WebSocket port ${port}`);
        ctx.fillStyle = "#111";
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = "#fff";
        ctx.fillText("✅ Connected. Waiting for first frame...", 20, 40);
      };

      ws.onmessage = async (event) => {
        const blob = event.data;
        const img = new Image();
        const url = URL.createObjectURL(blob);
        img.onload = () => {
          ctx.clearRect(0, 0, canvas.width, canvas.height);
          ctx.drawImage(img, 0, 0);
          URL.revokeObjectURL(url);
        };
        img.src = url;
      };

      ws.onerror = (err) => {
        console.error("[LAGOS] WebSocket Error:", err);
        ctx.fillStyle = "#f33";
        ctx.fillText("❌ WebSocket Error. Check /tmp/lagos.log", 20, 80);
      };

      // Telemetry Bridge: Send events back to the Rust daemon
      const sendEvent = (event) => {
        if (ws.readyState === WebSocket.OPEN) {
          ws.send(JSON.stringify(event));
        }
      };

      canvas.onmousedown = (e) => sendEvent({type: "MouseDown", x: e.offsetX, y: e.offsetY});
      canvas.onmousemove = (e) => sendEvent({type: "MouseMove", x: e.offsetX, y: e.offsetY});
      canvas.onmouseup = (e) => sendEvent({type: "MouseUp", x: e.offsetX, y: e.offsetY});
      canvas.onwheel = (e) => {
        e.preventDefault();
        sendEvent({type: "MouseWheel", delta: e.deltaY});
      };
    }
    """
    port = traitlets.Int(0).tag(sync=True)
    width = traitlets.Int(1000).tag(sync=True)
    height = traitlets.Int(400).tag(sync=True)

def plot_inline(handle_id, width=1000, height=400):
    """
    Spawns the Lagos Vision Daemon and returns an AnyWidget for inline display.
    """
    # 1. Retrieve the raw file descriptor from the Rust registry
    try:
        from . import data
        fd = data.get_fd(handle_id)
    except Exception as e:
        raise RuntimeError(f"Failed to retrieve FD for handle {handle_id}: {e}")

    # 2. Locate the bundled daemon binary
    bin_dir = Path(__file__).parent / "bin"
    daemon_path = bin_dir / "lagos-vision-daemon"

    if not daemon_path.exists():
        # Fallback for development mode
        dev_path = Path(__file__).parents[3] / "target" / "release" / "lagos-vision-daemon"
        if dev_path.exists():
            daemon_path = dev_path
        else:
            raise RuntimeError(f"Lagos Vision Daemon binary not found. Forge the wheel first!")

    # 3. Launch the daemon as a background process (Subprocess Purity)
    # We pass the FD explicitly to be inherited by the child process.
    logger.info(f"👁️ Spawning Lagos Vision Daemon (FD: {fd})")
    
    log_file = open("/tmp/lagos.log", "w")
    process = subprocess.Popen(
        [
            str(daemon_path), 
            "--fd", str(fd), 
            "--width", str(width), 
            "--height", str(height)
        ],
        stdout=subprocess.PIPE,
        stderr=log_file,
        text=True,
        pass_fds=[fd],
        start_new_session=True
    )

    # 4. Extract the dynamic port assigned by the OS
    port = None
    # We wait for the daemon to signal its port on stdout
    # Note: We need to read from the pipe which is now shared with log_file? 
    # No, stdout is still a PIPE.
    for _ in range(50): # Increased timeout
        line = process.stdout.readline()
        if not line:
            break
        # Still echo to log_file
        log_file.write(line)
        log_file.flush()
        
        if "[LAGOS_PORT:" in line:
            match = re.search(r"\[LAGOS_PORT: (\d+)\]", line)
            if match:
                port = int(match.group(1))
                break
        if process.poll() is not None:
            break

    if port is None:
        stderr = process.stderr.read()
        process.kill()
        raise RuntimeError(f"Lagos Vision Daemon failed to ignite. Stderr: {stderr}")

    logger.info(f"✅ Lagos Vision live on WebSocket port {port}")

    return LagosWidget(port=port, width=width, height=height)
