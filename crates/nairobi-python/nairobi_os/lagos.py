import anywidget
import traitlets
import subprocess
import os
import re
import sys
import logging
from pathlib import Path

logger = logging.getLogger(__name__)

class LagosWidget(anywidget.AnyWidget):
    """
    Lagos Vision Inline Widget for Jupyter Lab.
    Requires a physical display for rendering.
    """
    _esm = """
    export function render({ model, el }) {
      const img = document.createElement("img");
      img.style.width = "100%";
      img.style.height = "auto";
      img.src = model.get("image_url");
      el.appendChild(img);
    }
    """
    image_url = traitlets.Unicode("").tag(sync=True)

def plot_inline(handle_id, width=1000, height=400):
    raise RuntimeError(
        "lagos-lite headless visualization has been moved to the enterprise nairobi-lagos-vision package. "
        "For local display rendering, use the SovereignFrame.plot() method which requires a physical display. "
        "For headless/cloud environments (Google Colab, CI/CD), contact Sovereign Systems Lab for enterprise access."
    )