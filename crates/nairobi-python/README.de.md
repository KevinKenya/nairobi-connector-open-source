[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi Python

## Überblick
Nairobi Python bietet die High-Level-Brücke zur Nairobi OS-Infrastruktur. Es ermöglicht Datenwissenschaftlern, die Leistung von Rust-basierten, hardwarebeschleunigten Analysen über eine vertraute Python-Schnittstelle zu nutzen. Das Paket übernimmt das Daemon-Management, die IPC-Koordination und das Memory-Mapping, sodass sich die Benutzer auf die Datenanalyse konzentrieren können.

## Hauptmerkmale
- **SovereignFrame**: Eine flüssige, Pandas-ähnliche Schnittstelle zur Verwaltung von Remote-Daten-Handles.
- **Lazy Ignition**: Startet und konfiguriert den Refinery-Daemon automatisch beim ersten Datenzugriff.
- **Jupyter-Integration**: Erstklassige Unterstützung für interaktive Visualisierungen mit dem Lagos Vision Widget.
- **Zero-Copy Bridge**: Konsumiert direkt `memfd`-Handles von der Rust-Refinery mit einem Overhead von weniger als einer Millisekunde.

## Installation

### Von PyPI
```bash
pip install nairobi-os
```

### Aus dem Quellcode
```bash
cd crates/nairobi-python
pip install -e .
```
*Hinweis: Das Erstellen aus dem Quellcode erfordert die Rust-Toolchain und die Installation von `maturin`.*

## Verwendung

### Schnellstart
```python
import nairobi_os

# Mit der Refinery verbinden (übernimmt automatisch D-Bus und Daemon-Start)
nairobi_os.connect()

# Eine CSV-Datei aufnehmen
df = nairobi_os.read_csv("data.csv")

# Flüssige API für Statistiken
mean_val = df.column_name.mean()
p99_val = df.column_name.p99()

# SQL-Abfragen direkt auf der Engine ausführen
tall_players = df.query("SELECT * FROM dataset WHERE height > 80")

# Plotten mit Lagos Vision
tall_players.plot()
```

## API-Referenz

### `nairobi_os.connect()`
Initialisiert die Umgebung, startet bei Bedarf die D-Bus-Sitzung und zündet den Refinery-Daemon.

### `nairobi_os.read_csv(path, delimiter=",", encoding="utf-8")`
Nimmt eine CSV-Datei über die Zero-Copy-Pipeline der Refinery auf. Gibt einen `SovereignFrame` zurück.

### `SovereignFrame`-Methoden
- `df.column.mean()`: Berechnet das arithmetische Mittel.
- `df.column.std_dev()`: Berechnet die Standardabweichung.
- `df.column.p95()`, `df.column.p99()`: Berechnet Perzentile.
- `df.column.skewness()`, `df.column.kurtosis()`: Berechnet statistische Momente.
- `df.query(sql_string)`: Führt Polars-SQL auf dem Datensatz aus.
- `df.correlate("col1,col2")`: Berechnet Pearson- und Spearman-Korrelation.
- `df.plot(width, height)`: Zeigt eine interaktive `anywidget`-Visualisierung an.

## Entwicklung

### Hinzufügen neuer Python-Bindings
Nairobi Python verwendet PyO3 für die Schnittstelle zu Rust. Neue Kernfunktionen sollten in `crates/nairobi-python/src/lib.rs` hinzugefügt und über das Modul `nairobi_os._core.data` exponiert werden.

### Testen
Integrationstests für das Python-Paket können mit `pytest` (falls konfiguriert) oder dem bereitgestellten Testskript ausgeführt werden:
```bash
python3 test_nairobi.py
```

Um isoliert ohne die vollständige Refinery zu testen, können Sie das Modul `_core.data` mocken oder den `SovereignFrame` mit bereits vorhandenen Handles verwenden.

## Fehlerbehebung
- **Refinery konnte nicht auf D-Bus registriert werden**: Dies geschieht häufig in Headless-Umgebungen. Stellen Sie sicher, dass `dbus-launch` verfügbar ist, oder rufen Sie `nairobi_os.connect()` auf, das versucht, die Umgebung zu reparieren.
- **Handle nicht gefunden**: Daten-Handles sind an die Sitzung gebunden. Wenn die Refinery neu startet, werden vorherige `SovereignFrame`-Handles ungültig.

## Unterstützung
Wenn Sie Nairobi OS nützlich finden, ziehen Sie bitte in Erwägung, das Projekt zu unterstützen:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Lizenz
Dieses Projekt ist unter der **Apache License 2.0** lizenziert.

---
© 2026 Kevin Chege. Alle Rechte vorbehalten.
