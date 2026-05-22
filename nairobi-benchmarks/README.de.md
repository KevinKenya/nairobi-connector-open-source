[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi Benchmark Suite

## Überblick
Die Nairobi Benchmark Suite ist ein strenges Framework zur Leistungsbewertung, das darauf ausgelegt ist, Nairobi OS mit Industriestandard-Datenverarbeitungsbibliotheken (z. B. Pandas) zu vergleichen. Sie konzentriert sich auf End-to-End-Latenz, Speichereffizienz und die Auswirkungen von "Fused Analytical Strikes" auf reale Workloads.

## Hauptkennzahlen
- **Ingestion Latency**: Zeit zum Laden von Daten von der Festplatte in speicherresidente Strukturen.
- **Compute Density**: Peak Resident Set Size (RSS) während schwerer analytischer Lasten.
- **Pipeline Throughput**: Gesamtzeit für kombinierte Ingest-Crunch-Correlate-Operationen.

## Installation

### Voraussetzungen
- Python 3.10+
- Nairobi OS (installiert und konfiguriert)

### Setup
```bash
cd nairobi-benchmarks
pip install -r requirements.txt
```

## Benchmarks ausführen

### 1. Datensätze vorbereiten
Erzeugen Sie synthetische Datensätze, um die Skalierung zu testen:
```bash
# 10M Zeilen Datensatz generieren
python datasets/generators/generate_synthetic.py --type tall --output datasets/synthetic/tall_10m.csv
```

### 2. Workloads ausführen
Führen Sie einen spezifischen Benchmark-Workload aus:
```bash
python orchestration/benchmark_runner.py --workload workloads/workload_statistical_distillation.yaml --iterations 10
```

### 3. Ergebnisse analysieren
Benchmark-Ergebnisse werden im JSON-Format gespeichert und können mit den enthaltenen Plotting-Tools visualisiert werden:
```bash
python visualization/plot_scaling.py
```

## Methodik
Die Suite folgt einer "Hardware-First" Benchmarking-Methodik, die sicherstellt, dass:
- Kalt- und Warmstarts separat gemessen werden.
- Kernel-Caches (wo möglich) zwischen den Läufen geleert werden.
- Alle Berechnungen mit `result_validator.py` auf mathematische Identität (±1e-5) überprüft werden.

## Unterstützung
Wenn Sie Nairobi OS nützlich finden, ziehen Sie bitte in Erwägung, das Projekt zu unterstützen:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Lizenz
Diese Suite ist Teil des Nairobi OS-Projekts und ist unter der **Apache License 2.0** lizenziert.

---
© 2026 Kevin Chege. Alle Rechte vorbehalten.
