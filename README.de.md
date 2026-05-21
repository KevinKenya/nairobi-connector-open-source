[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi OS

## Überblick
Nairobi OS ist eine leistungsstarke, verteilte KI- und Data-Science-Infrastruktur, die für extreme Ressourceneffizienz entwickelt wurde. Es ermöglicht die Verarbeitung massiver Datensätze in eingeschränkten Umgebungen (Edge, IoT, Serverless) durch einen spezialisierten Rust-basierten Refinery-Daemon und bietet **Computernutzung ohne Pixel** über seine MCP-kompatible Accessibility-Brücke.

Durch die Nutzung von Kernel-Level-Funktionen wie `io_uring`, `memfd` und Huge Pages erreicht Nairobi OS einen IPC-Overhead im Sub-Millisekundenbereich und Zero-Copy-Datenpipelines.

## Demo

https://github.com/user-attachments/assets/demo.webm

<video src="assets/demo.webm" controls width="100%">
  Ihr Browser unterstützt das Video-Tag nicht.
</video>

## Hauptfunktionen
- **Computernutzung ohne Pixel**: Interagiert direkt über AT-SPI2 und den TOON (Token-Oriented Object Notation)-Komprimierungsalgorithmus mit dem Linux-Desktop.
- **Zero-Copy-Ingestion**: Hardwarebeschleunigtes Datenladen über `io_uring` und 1GB Huge Pages.
- **Hardwarebeschleunigte Visualisierung**: Interaktives Jupyter-Plotting über die Lagos Vision-Engine.
- **Fused Analytics Pipeline**: Daten in einem einzigen D-Bus-Umlauf aufnehmen und korrelieren.
- **Sovereign Interface**: Eine fließende Python-API.

## Architektur
1.  **Nairobi Axum Refinery**: Der leistungsstarke Rust-Kern.
2.  **Nairobi Hub**: Der IPC-Orchestrator.
3.  **Lagos Vision**: Die Headless-Rendering-Engine.
4.  **Nairobi Connector**: Die semantische Brücke (MCP Server).
5.  **Nairobi Python**: Die High-Level-Brücke zur Rust-Umgebung.

## Installation

### Von PyPI
```bash
pip install nairobi-os
```

### Aus dem Quellcode kompilieren
```bash
git clone https://github.com/KevinKenya/nairobi-connector-open-source
cd nairobi-connector-open-source
python3 -m venv .venv
source .venv/bin/activate
pip install maturin pyo3-build-config zbus anywidget traitlets
./build_wheel.sh
```

## Nutzung

### Datenanalytik
```python
import nairobi_os

nairobi_os.connect()
df = nairobi_os.read_csv("dataset.csv")
print(f"Mittelwert: {df.column_name.mean()}")
df.plot()
```

### Computernutzung (MCP Server)
KI-Agenten sollten dieser Schleife folgen:
1. Zielen Sie auf ein Fenster mit `nairobi_find_window`.
2. Beobachten Sie den Status über `nairobi_get_ui_map`.
3. Lesen Sie die `[ID: N]` des gewünschten Elements ab.
4. Führen Sie eine Aktion über `nairobi_interact` oder `nairobi_type_text` aus.

## Lizenz
Dieses Projekt ist unter der **Apache License 2.0** lizenziert.
*(Hinweis: Teile des TOON-Formats gehen auf die TOON-Autoren zurück).*

---
© 2026 Kevin Chege. Alle Rechte vorbehalten.
