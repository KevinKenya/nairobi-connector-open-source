# File: /home/chege/nairobi-connector-open-source/crates/nairobi-python/nairobi_os/widget.py
# Author: Jules (Lead Graphics Architect)
# Date: 2026-05-10

import anywidget
import traitlets

class LagosPlotWidget(anywidget.AnyWidget):
    _esm = """
    function render({ model, el }) {
        const canvas = document.createElement("canvas");
        canvas.style.width = "100%";
        canvas.style.height = "100%";
        canvas.style.display = "block";
        canvas.style.background = "#111";
        el.appendChild(canvas);

        const ctx = canvas.getContext("2d");
        let ws = null;
        let isDragging = false;

        function connect() {
            const port = model.get("ws_port");
            if (!port) return;

            ws = new WebSocket(`ws://127.0.0.1:${port}`);
            ws.binaryType = "arraybuffer";

            ws.onmessage = (event) => {
                if (event.data instanceof ArrayBuffer) {
                    const blob = new Blob([event.data], { type: "image/jpeg" });
                    const url = URL.createObjectURL(blob);
                    const img = new Image();
                    img.onload = () => {
                        // Match canvas internal resolution to CSS size if needed
                        if (canvas.width !== img.width || canvas.height !== img.height) {
                            canvas.width = img.width;
                            canvas.height = img.height;
                        }
                        ctx.drawImage(img, 0, 0);
                        URL.revokeObjectURL(url);
                    };
                    img.src = url;
                }
            };

            ws.onclose = () => {
                console.log("Lagos WebSocket closed. Retrying...");
                setTimeout(connect, 1000);
            };
        }

        // Telemetry TX
        const sendTelemetry = (data) => {
            if (ws && ws.readyState === WebSocket.OPEN) {
                ws.send(JSON.stringify(data));
            }
        };

        canvas.addEventListener("mousedown", (e) => {
            isDragging = true;
            const rect = canvas.getBoundingClientRect();
            const x = (e.clientX - rect.left) * (canvas.width / rect.width);
            const y = (e.clientY - rect.top) * (canvas.height / rect.height);
            sendTelemetry({ t: "c", x, y, pressed: true, button: e.button });
        });

        window.addEventListener("mouseup", (e) => {
            if (isDragging) {
                isDragging = false;
                const rect = canvas.getBoundingClientRect();
                const x = (e.clientX - rect.left) * (canvas.width / rect.width);
                const y = (e.clientY - rect.top) * (canvas.height / rect.height);
                sendTelemetry({ t: "c", x, y, pressed: false, button: e.button });
            }
        });

        canvas.addEventListener("mousemove", (e) => {
            const rect = canvas.getBoundingClientRect();
            const x = (e.clientX - rect.left) * (canvas.width / rect.width);
            const y = (e.clientY - rect.top) * (canvas.height / rect.height);
            sendTelemetry({ t: "d", x, y, pressed: isDragging });
        });

        canvas.addEventListener("wheel", (e) => {
            e.preventDefault();
            sendTelemetry({ t: "z", dy: -e.deltaY / 10.0 });
        }, { passive: false });

        model.on("change:ws_port", connect);
        connect();
    }
    export default { render };
    """
    ws_port = traitlets.Int(0).tag(sync=True)
