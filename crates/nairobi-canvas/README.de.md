[English](README.md) | [简体中文](README.zh-CN.md) | [廣東話](README.yue.md) | [Français](README.fr.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Suomi](README.fi.md) | [Nederlands](README.nl.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Canvas: Visueller Immediate-Mode Knotengraph-Compiler

Nairobi Canvas ist ein hardwarebeschleunigter visueller Compiler zum Erstellen von Datenverarbeitungspipelines. Er bietet eine im Immediate-Mode gestaltete Knoten-UI basierend auf `egui`/`egui-snarl`, die visuelle Arbeitsabläufe in das GVariant DAG-Format (Directed Acyclic Graph) zur Ausführung durch den Nairobi Hub kompiliert.

## Funktionen

- **Visueller Pipeline-Builder**: Drag-and-Drop-Knotengraph-Interface für Daten-Workflows.
- **Nativer Dateimanager**: Klicken Sie auf die Schaltfläche 📂 bei Ingest-Knoten, um nach CSV-Dateien zu suchen.
- **SQL-Abfrage-Presets**: Vorkonfigurierte Abfragevorlagen (Alle Spalten, Einzelne Spalte, Where-Klausel, Mehrspaltig).
- **GVariant-Serialisierung**: Kompiliert Graphen in das GVariant-Format für Zero-Copy IPC.
- **Topologische Sortierung**: Automatische Zyklenerkennung und Ausführungsreihenfolge.

## Knotentypen

| Knoten | Eingaben | Ausgaben | Beschreibung |
|--------|---------|----------|--------------|
| **Ingest** | 0 | 1 | Lädt CSV-Datensätze über den nativen Dateimanager |
| **SqlQuery** | 1 | 1 | Führt Polars-SQL-Abfragen auf Eingabedaten aus |
| **AxiomCrunch** | 1 | 1 | Berechnet Statistiken (Mittelwert, Standardabweichung, Kurtosis) |
| **LagosPlot** | 1 | 0 | Rendert Visualisierungen (Sparkline, Scatter, PNG, JPG) |

## Installation

```bash
pip install nairobi-os
```

Oder aus dem Quellcode bauen:
```bash
cargo build --release
# Die Canvas-Demo ist eine Rust-Binärdatei - siehe examples/canvas_compile_demo.rs
```

## Verwendung

### Rust (Nativ)

Führen Sie die Demo-Anwendung aus:
```bash
cargo run --example canvas_compile_demo
```

### Python

Verwendung des installierten Pakets:
```python
import nairobi_os as nb

# Öffnet das visuelle Canvas für die DAG-Kompilierung
dag_bytes = nb.canvas.open()

# Führt die kompilierte Pipeline aus
if dag_bytes:
    nb.canvas.execute(dag_bytes)
```

Oder führen Sie das vollständige Testskript aus:
```bash
python test_canvas.py
```

Dieses Skript führt folgende Schritte aus:
1. `nairobi_os.ignite()` - Startet die Daemons Axum Refinery und Nairobi Hub.
2. `nb.canvas.open()` - Startet den visuellen Knotengraph-Editor.
3. `nb.canvas.execute(dag_bytes)` - Führt die kompilierte Pipeline mit Zeitmessungen aus.

Das Canvas exportiert einen GVariant-kodierten DAG, der:
- über `nb.canvas.execute()` ausgeführt werden kann.
- zur späteren Verwendung auf Festplatte gespeichert werden kann.
- über D-Bus/Shared Memory übertragen werden kann.

## Graphen erstellen

1. **Rechtsklick** auf das Canvas-Gitter, um das Knotenmenü zu öffnen.
2. Wählen Sie einen Knotentyp (Ingest, SQL Query, Axiom Crunch oder Lagos Plot).
3. **Verbinden** Sie Knoten, indem Sie von den Ausgangs-Pins (blau) zu den Eingangs-Pins (grün) ziehen.
4. Klicken Sie auf **Compile Graph**, um den Workflow zu serialisieren.

## Ausführungsfluss

```
Canvas-Graph → GVariant-DAG → Nairobi Hub → Axum Refinery / Lagos Vision
```

Der kompilierte DAG wird via IPC an den Hub übertragen, der die Knoten weiterleitet an:
- **Axum Refinery**: Datenaufnahme und statistische Verarbeitung.
- **Lagos Vision**: Hardwarebeschleunigtes Rendering von Visualisierungen.

Für Architekturdetails und die vollständige Systemübersicht siehe die [README des Haupt-Repositories](../README.md).

## Support
Wenn Sie Nairobi OS nützlich finden, ziehen Sie bitte eine Unterstützung des Projekts in Betracht:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Lizenz
Dieses Projekt ist unter der **Apache License 2.0** lizenziert.

© 2026 Kevin Chege. Alle Rechte vorbehalten.
