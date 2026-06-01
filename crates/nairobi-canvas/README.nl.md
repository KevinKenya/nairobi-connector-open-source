[English](README.md) | [简体中文](README.zh-CN.md) | [廣東話](README.yue.md) | [Français](README.fr.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Suomi](README.fi.md) | [Nederlands](README.nl.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Canvas: Immediate-Mode Node Graph Visuele Compiler

Nairobi Canvas is een hardware-versnelde visuele compiler voor het bouwen van dataprocessing-pipelines. Het biedt een immediate-mode node graph UI, gebouwd op `egui`/`egui-snarl`, die visuele workflows compileert naar GVariant DAG (Directed Acyclic Graph) formaat voor uitvoering door de Nairobi Hub.

## Kenmerken

- **Visuele Pipeline Builder**: Drag-and-drop node graph interface voor dataworkflows
- **Native File Picker**: Klik op de 📂 knop op Ingest-nodes om naar CSV-bestanden te zoeken
- **SQL Query Presets**: Geconfigureerde query-sjablonen (Alle kolommen, Enkele kolom, Where-clausule, Multi-kolom)
- **GVariant Serialisatie**: Compileert grafieken naar GVariant-formaat voor zero-copy IPC
- **Topologische Sortering**: Automatische cyclusdetectie en uitvoeringsvolgorde

## Node Types

| Node | Inputs | Outputs | Beschrijving |
|------|--------|---------|-------------|
| **Ingest** | 0 | 1 | Laadt CSV-datasets via de native file picker |
| **SqlQuery** | 1 | 1 | Voert Polars SQL-queries uit op invoerdata |
| **AxiomCrunch** | 1 | 1 | Berekent statistieken (gemiddelde, standaarddeviatie, kurtosis) |
| **LagosPlot** | 1 | 0 | Rendert visualisaties (sparkline, scatter, PNG, JPG) |

## Installatie

```bash
pip install nairobi-os
```

Of bouw vanaf broncode:
```bash
cargo build --release
# De canvas demo is een Rust binary - zie examples/canvas_compile_demo.rs
```

## Gebruik

### Rust (Native)

Voer de demo-applicatie uit:
```bash
cargo run --example canvas_compile_demo
```

### Python

Gebruik het geïnstalleerde pakket:
```python
import nairobi_os as nb

# Open de visuele canvas voor DAG-compilatie
dag_bytes = nb.canvas.open()

# Voer de gecompileerde pipeline uit
if dag_bytes:
    nb.canvas.execute(dag_bytes)
```

Of voer het complete testscript uit:
```bash
python test_canvas.py
```

Dit script voert uit:
1. `nairobi_os.ignite()` - Start Axum Refinery en Nairobi Hub daemons
2. `nb.canvas.open()` - Start de visuele node graph editor
3. `nb.canvas.execute(dag_bytes)` - Voert de gecompileerde pipeline uit met timing-metrieken

De canvas exporteert een GVariant-gecodeerde DAG die kan worden:
- Uitgevoerd via `nb.canvas.execute()`
- Opgeslagen op schijf voor later gebruik
- Verzonden via D-Bus/gedeeld geheugen

## Grafieken Bouwen

1. **Rechtermuisklik** op het canvas raster om het node-menu te openen
2. Selecteer een node-type (Ingest, SQL Query, Axiom Crunch of Lagos Plot)
3. **Verbind** nodes door te slepen van output-pinnen (blauw) naar input-pinnen (groen)
4. Klik op **Compile Graph** om de workflow te serialiseren

## Uitvoeringsstroom

```
Canvas Grafiek → GVariant DAG → Nairobi Hub → Axum Refinery / Lagos Vision
```

De gecompileerde DAG wordt via IPC verzonden naar de Hub, die nodes routeert naar:
- **Axum Refinery**: Data-ingestie en statistische verwerking
- **Lagos Vision**: Hardware-versnelde visualisatie-rendering

Zie de [README van de hoofdrepository](../README.md) voor architectuurdetails en het volledige systeemoverzicht.

## Ondersteuning
Als je Nairobi OS nuttig vindt, overweeg dan om het project te steunen:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Licentie
Dit project is gelicentieerd onder de **Apache License 2.0**.

© 2026 Kevin Chege. Alle rechten voorbehouden.
