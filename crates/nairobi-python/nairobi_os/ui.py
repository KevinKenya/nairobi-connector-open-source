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

"""
Nairobi OS — Computer Use Interface (Action Engine)

Python MCP client that manages the nairobi-connector binary over stdio,
providing semantic UI control via AT-SPI2 accessibility tree traversal.

Usage:
    >>> import nairobi_os
    >>>
    >>> # Start the MCP server
    >>> nairobi_os.ui.start()
    >>>
    >>> # Find and target a window
    >>> nairobi_os.ui.find_window("Text Editor")
    >>>
    >>> # Get the UI map (TOON format)
    >>> toon = nairobi_os.ui.get_map()
    >>> print(toon)
    >>>
    >>> # Interact with a node
    >>> nairobi_os.ui.interact(node_id=3, action="click")
    >>>
    >>> # Type text into a field
    >>> nairobi_os.ui.type_text(node_id=5, text="Hello, World!")
    >>>
    >>> # Stop the server
    >>> nairobi_os.ui.stop()

Context Manager:
    >>> with nairobi_os.ui as u:
    ...     u.find_window("Firefox")
    ...     toon = u.get_map()
    ...     print(toon)
"""

import json
import logging
import os
import shutil
import subprocess
import threading
import time
from pathlib import Path

logger = logging.getLogger(__name__)

# JSON-RPC 2.0 constants
_JSONRPC_VERSION = "2.0"

# Default timeouts
_STARTUP_TIMEOUT = 10  # seconds to wait for MCP server initialization
_CALL_TIMEOUT = 30  # seconds to wait for a tool call response


class MCPClientError(Exception):
    """Raised when the MCP client encounters a protocol or transport error."""
    pass


class _UIModule:
    """Computer Use interface via Nairobi Connector MCP server.

    Manages the nairobi-connector binary as a subprocess communicating
    over stdio using the MCP (Model Context Protocol) JSON-RPC 2.0 transport.
    """

    def __init__(self):
        self._process = None
        self._request_id = 0
        self._lock = threading.Lock()
        self._initialized = False
        self._server_info = None

    # ─────────────────────────────────────────────────────────────────────
    # Lifecycle
    # ─────────────────────────────────────────────────────────────────────

    def start(self, binary_path=None, timeout=_STARTUP_TIMEOUT):
        """Start the MCP server subprocess and perform the initialize handshake.

        Args:
            binary_path: Optional explicit path to the nairobi-connector binary.
                         If None, searches in nairobi_os/bin/ then $PATH.
            timeout: Seconds to wait for the initialize handshake to complete.

        Returns:
            True if the server started and initialized successfully.

        Raises:
            MCPClientError: If the binary is not found or handshake fails.
        """
        if self._process is not None and self._process.poll() is None:
            logger.info("MCP server already running (PID: %d)", self._process.pid)
            return True

        binary = self._resolve_binary(binary_path)
        logger.info("🚀 Starting Nairobi Connector MCP server: %s", binary)

        try:
            self._process = subprocess.Popen(
                [str(binary)],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                start_new_session=True,
            )
        except OSError as e:
            raise MCPClientError(f"Failed to start nairobi-connector: {e}") from e

        logger.info("MCP server started (PID: %d)", self._process.pid)

        # Perform MCP initialize handshake
        try:
            self._do_initialize(timeout)
        except Exception as e:
            self.stop()
            raise MCPClientError(f"MCP initialize handshake failed: {e}") from e

        self._initialized = True
        logger.info("✅ MCP server initialized — tools ready")
        return True

    def stop(self):
        """Stop the MCP server subprocess."""
        if self._process is not None:
            try:
                self._process.terminate()
                self._process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                self._process.kill()
                self._process.wait(timeout=2)
            except Exception:
                pass
            finally:
                self._process = None
                self._initialized = False
                self._request_id = 0
                logger.info("🛑 MCP server stopped")

    def is_running(self):
        """Check if the MCP server is running."""
        return self._process is not None and self._process.poll() is None

    # ─────────────────────────────────────────────────────────────────────
    # Context Manager
    # ─────────────────────────────────────────────────────────────────────

    def __enter__(self):
        self.start()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.stop()
        return False

    # ─────────────────────────────────────────────────────────────────────
    # MCP Tools — Public API
    # ─────────────────────────────────────────────────────────────────────

    def find_window(self, title, timeout=_CALL_TIMEOUT):
        """Find and target a window by title substring (case-insensitive).

        Must be called before get_map() to set which window to inspect.

        Args:
            title: Substring to match against window titles.
            timeout: Seconds to wait for response.

        Returns:
            dict with 'success' and 'message' keys.

        Raises:
            MCPClientError: If the call fails or times out.
        """
        self._ensure_running()
        return self._call_tool("nairobi_find_window", {"title": title}, timeout)

    def get_map(self, max_depth=7, timeout=_CALL_TIMEOUT):
        """Get the current UI accessibility tree as a TOON-compressed map.

        Returns a hyper-dense Markdown listing of all interactive elements
        with sequential IDs for action targeting.

        Args:
            max_depth: Maximum depth to traverse (default: 7).
            timeout: Seconds to wait for response.

        Returns:
            str — the TOON-formatted accessibility tree.

        Raises:
            MCPClientError: If no window is targeted or the call fails.
        """
        self._ensure_running()
        result = self._call_tool(
            "nairobi_get_ui_map", {"max_depth": max_depth}, timeout
        )
        # Extract the TOON string from structured response
        if isinstance(result, dict) and "toon" in result:
            return result["toon"]
        # Fallback: return raw text content
        return str(result)

    def interact(self, node_id, action="click", timeout=_CALL_TIMEOUT):
        """Execute a semantic action on a UI element by TOON node ID.

        Args:
            node_id: The TOON node ID (from the last get_map() output).
            action: One of "click", "activate", or "focus".
            timeout: Seconds to wait for response.

        Returns:
            dict with 'success' and 'message' keys.

        Raises:
            MCPClientError: If the action is invalid or fails.
        """
        self._ensure_running()
        return self._call_tool(
            "nairobi_interact", {"node_id": node_id, "action": action}, timeout
        )

    def type_text(self, node_id, text, timeout=_CALL_TIMEOUT):
        """Inject text into an editable field by TOON node ID.

        Atomically replaces all text in the element.

        Args:
            node_id: The TOON node ID of the editable field.
            text: The text to inject.
            timeout: Seconds to wait for response.

        Returns:
            dict with 'success' and 'message' keys.

        Raises:
            MCPClientError: If the field is not editable or the call fails.
        """
        self._ensure_running()
        return self._call_tool(
            "nairobi_type_text", {"node_id": node_id, "text": text}, timeout
        )

    # ─────────────────────────────────────────────────────────────────────
    # MCP Protocol — JSON-RPC 2.0 over stdio
    # ─────────────────────────────────────────────────────────────────────

    def _do_initialize(self, timeout):
        """Perform the MCP initialize handshake.

        Sends an 'initialize' request and waits for the server's capabilities
        response, then sends the 'initialized' notification.
        """
        # Step 1: Send initialize request
        init_params = {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "nairobi-os-python",
                "version": "0.4.0",
            },
        }
        response = self._send_request("initialize", init_params, timeout)

        self._server_info = response.get("result", {})
        logger.debug("Server info: %s", self._server_info)

        # Step 2: Send initialized notification (no ID = notification)
        self._send_notification("notifications/initialized", {})

    def _call_tool(self, tool_name, arguments, timeout):
        """Call an MCP tool and return the parsed result.

        Args:
            tool_name: The tool name (e.g., "nairobi_get_ui_map").
            arguments: Dict of tool arguments.
            timeout: Seconds to wait for response.

        Returns:
            Parsed result — either a dict (structured) or str (text content).

        Raises:
            MCPClientError: On protocol errors or timeout.
        """
        params = {"name": tool_name, "arguments": arguments}
        response = self._send_request("tools/call", params, timeout)

        if "error" in response:
            error = response["error"]
            raise MCPClientError(
                f"Tool '{tool_name}' error [{error.get('code', '?')}]: "
                f"{error.get('message', 'Unknown error')}"
            )

        result = response.get("result", {})

        # Try to parse structured content
        content_list = result.get("content", [])
        if content_list:
            first = content_list[0]
            if first.get("type") == "text":
                text = first.get("text", "")
                # Try to parse as JSON (structured response)
                try:
                    return json.loads(text)
                except (json.JSONDecodeError, TypeError):
                    return text

        # Check for isError flag
        if result.get("isError"):
            text_parts = [
                c.get("text", "") for c in content_list if c.get("type") == "text"
            ]
            raise MCPClientError(f"Tool '{tool_name}' failed: {' '.join(text_parts)}")

        return result

    def _send_request(self, method, params, timeout):
        """Send a JSON-RPC 2.0 request and wait for the response.

        Args:
            method: The RPC method name.
            params: The request parameters.
            timeout: Seconds to wait.

        Returns:
            The parsed JSON-RPC response dict.

        Raises:
            MCPClientError: On write/read errors or timeout.
        """
        with self._lock:
            self._request_id += 1
            req_id = self._request_id

        request = {
            "jsonrpc": _JSONRPC_VERSION,
            "id": req_id,
            "method": method,
            "params": params,
        }

        self._write_message(request)
        return self._read_response(req_id, timeout)

    def _send_notification(self, method, params):
        """Send a JSON-RPC 2.0 notification (no response expected).

        Args:
            method: The RPC method name.
            params: The notification parameters.
        """
        notification = {
            "jsonrpc": _JSONRPC_VERSION,
            "method": method,
            "params": params,
        }
        self._write_message(notification)

    def _write_message(self, message):
        """Write a JSON-RPC message to the server's stdin.

        Args:
            message: Dict to serialize as JSON.

        Raises:
            MCPClientError: If the process is dead or write fails.
        """
        if not self.is_running():
            raise MCPClientError("MCP server is not running")

        try:
            data = json.dumps(message) + "\n"
            self._process.stdin.write(data.encode("utf-8"))
            self._process.stdin.flush()
            logger.debug(">>> %s", data.strip())
        except (BrokenPipeError, OSError) as e:
            raise MCPClientError(f"Failed to write to MCP server: {e}") from e

    def _read_response(self, expected_id, timeout):
        """Read a JSON-RPC response matching the expected request ID.

        Skips notifications and server-initiated messages until the
        matching response is found or timeout expires.

        Args:
            expected_id: The request ID to match.
            timeout: Seconds to wait.

        Returns:
            The parsed JSON-RPC response dict.

        Raises:
            MCPClientError: On timeout, EOF, or parse errors.
        """
        deadline = time.monotonic() + timeout

        while time.monotonic() < deadline:
            if not self.is_running():
                # Grab any stderr output for diagnostics
                stderr_out = ""
                try:
                    stderr_out = self._process.stderr.read().decode("utf-8", errors="replace")
                except Exception:
                    pass
                raise MCPClientError(
                    f"MCP server died unexpectedly. stderr: {stderr_out[:500]}"
                )

            try:
                line = self._process.stdout.readline()
                if not line:
                    # EOF — server closed
                    raise MCPClientError("MCP server closed stdout (EOF)")

                line_str = line.decode("utf-8").strip()
                if not line_str:
                    continue

                logger.debug("<<< %s", line_str[:200])
                message = json.loads(line_str)

                # Skip notifications (no 'id' field)
                if "id" not in message:
                    logger.debug("Skipping notification: %s", message.get("method", "?"))
                    continue

                # Check if this is the response we're waiting for
                if message.get("id") == expected_id:
                    return message

                # Unexpected ID — log and continue
                logger.warning(
                    "Unexpected response ID %s (expected %s)",
                    message.get("id"),
                    expected_id,
                )

            except json.JSONDecodeError as e:
                logger.warning("Invalid JSON from MCP server: %s", e)
                continue

        raise MCPClientError(
            f"Timeout waiting for response to request {expected_id} "
            f"(waited {timeout}s)"
        )

    # ─────────────────────────────────────────────────────────────────────
    # Internal Helpers
    # ─────────────────────────────────────────────────────────────────────

    def _ensure_running(self):
        """Auto-start the MCP server if not already running."""
        if not self.is_running():
            self.start()

    def _resolve_binary(self, explicit_path=None):
        """Resolve the nairobi-connector binary path.

        Search order:
        1. Explicit path argument
        2. nairobi_os/bin/nairobi-connector (bundled in wheel)
        3. $PATH lookup

        Args:
            explicit_path: Optional explicit binary path.

        Returns:
            Path to the binary.

        Raises:
            MCPClientError: If the binary cannot be found.
        """
        if explicit_path is not None:
            path = Path(explicit_path)
            if path.exists():
                return path
            raise MCPClientError(f"Binary not found at explicit path: {path}")

        # Check bundled location (inside the nairobi_os package)
        bundled = Path(__file__).parent / "bin" / "nairobi-connector"
        if bundled.exists():
            logger.debug("Using bundled binary: %s", bundled)
            return bundled

        # Fallback to $PATH
        system_bin = shutil.which("nairobi-connector")
        if system_bin:
            logger.debug("Using system binary: %s", system_bin)
            return Path(system_bin)

        # Last resort: check workspace target/release (dev environment)
        workspace_bin = (
            Path(__file__).parent.parent.parent.parent.parent
            / "target"
            / "release"
            / "nairobi-connector"
        )
        if workspace_bin.exists():
            logger.debug("Using workspace binary: %s", workspace_bin)
            return workspace_bin

        raise MCPClientError(
            "nairobi-connector binary not found. "
            "Build it with: cargo build --release -p nairobi-connector"
        )

    def __repr__(self):
        status = "running" if self.is_running() else "stopped"
        pid = self._process.pid if self._process else None
        return f"<nairobi_os.ui ({status}, pid={pid})>"


# Module-level singleton instance
ui = _UIModule()
