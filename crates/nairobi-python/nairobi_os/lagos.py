# File: /home/chege/nairobi-connector-open-source/crates/nairobi-python/nairobi_os/lagos.py
# Author: Kevin Chege, Location: Nairobi, Date: 10th May 2026

import os
import subprocess
import threading
import time
import re
from ._core import get_fd
from .widget import LagosPlotWidget

def plot_inline(handle_id, width=1280, height=720):
    """
    Spawns the Lagos Vision Daemon and returns a Jupyter widget.

    Args:
        handle_id: The UUID handle from nairobi_os.data.ingest()
        width: Internal render width
        height: Internal render height
    """
    # 1. Retrieve the raw FD from the Rust registry
    # Note: get_fd() dups the FD, so we are responsible for it or passing it to a subprocess
    fd = get_fd(handle_id)

    # 2. Locate the daemon binary
    # It should be in nairobi_os/bin/ relative to this file
    base_dir = os.path.dirname(__file__)
    daemon_bin = os.path.join(base_dir, "bin", "lagos-vision-daemon")

    if not os.path.exists(daemon_bin):
        # Fallback for development environments where build_wheel.sh hasn't run but target/ exists
        dev_bin = os.path.join(base_dir, "..", "..", "..", "target", "release", "lagos-vision-daemon")
        if os.path.exists(dev_bin):
            daemon_bin = dev_bin
        else:
            raise FileNotFoundError(f"Lagos Vision Daemon not found at {daemon_bin}")

    # 3. Spawn the daemon
    # We pass the FD to the subprocess. On Linux, we can use pass_fds.
    # The daemon expects --handle <FD_NUMBER>
    proc = subprocess.Popen(
        [
            daemon_bin,
            "--handle", str(fd),
            "--width", str(width),
            "--height", str(height),
            "--port", "0" # Bind to dynamic port
        ],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        pass_fds=[fd],
        bufsize=1
    )

    # We can close our copy of the FD now that it's inherited by the child
    os.close(fd)

    # 4. Capture the port handshake
    # Pattern: [LAGOS_PORT: <PORT>]
    port = None

    # Wait for the handshake in a non-blocking-ish way (with timeout)
    start_time = time.time()
    while time.time() - start_time < 10:
        line = proc.stdout.readline()
        if not line:
            break

        match = re.search(r"\[LAGOS_PORT: (\d+)\]", line)
        if match:
            port = int(match.group(1))
            break

    if port is None:
        proc.kill()
        raise RuntimeError("Failed to capture Lagos Vision Daemon handshake. Check stderr.")

    # 5. Instantiate and return the widget
    widget = LagosPlotWidget()
    widget.ws_port = port

    # Store process reference to prevent premature GC
    widget._daemon_proc = proc

    return widget
