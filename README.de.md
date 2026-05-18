[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi OS

## Überblick
Nairobi OS ist eine verteilte Hochleistungs-Datenwissenschafts-Infrastruktur, die für extreme Ressourceneffizienz entwickelt wurde. Sie ermöglicht die Verarbeitung massiver Datensätze in eingeschränkten Umgebungen (Edge, IoT, Serverless) durch den Einsatz eines spezialisierten Rust-basierten Refinery-Daemons. Durch die Nutzung von Kernel-Level-Features wie `io_uring`, `memfd` und Huge Pages erreicht Nairobi OS einen IPC-Overhead von weniger als einer Millisekunde und Zero-Copy-Datenpipelines.

## Hauptmerkmale
- **Zero-Copy Ingestion**: Hardwarebeschleunigtes Laden von Daten mit `io_uring` und 1GB Huge Pages.
- **Hardwarebeschleunigte Visualisierung**: Interaktives Jupyter-Plotting über die Lagos Vision Engine (`wgpu` und `egui`).
- **Fused Analytics Pipeline**: Daten in einem einzigen D-Bus-Roundtrip aufnehmen, verarbeiten und korrelieren.
- **Kernel-Bypass-Performance**: Vektorisierte Analytik unter Nutzung von Polars und Rayon für maximale Hardwaresättigung.
- **Sovereign Interface**: Eine flüssige Python-API, die die Komplexität von Low-Level-IPC und Speicherverwaltung verbirgt.

## Architektur
Nairobi OS basiert auf einer Triade spezialisierter Komponenten, die über D-Bus und Shared Memory verbunden sind:

1.  **Nairobi Axum Refinery**: Der Rust-Hochleistungskern. Verwaltet die Rohdatenaufnahme und parallelisierte Analytik.
2.  **Nairobi Hub**: Der IPC-Orchestrator. Koordiniert Dateideskriptoren und Signale zwischen der Refinery und den Clients.
3.  **Lagos Vision**: Der visuelle Cortex. Eine Headless-Rendering-Engine, die `memfd`-Handles direkt in die GPU-Pipeline abbildet.
4.  **Nairobi Python**: Die High-Level-Brücke. Bietet eine Python-Schnittstelle für das Rust-Ökosystem.

```text
[ Datenquelle ] -> (io_uring/Huge Pages) -> [ Axum Refinery ]
                                                   |
                                          (D-Bus / memfd / iceoryx2)
                                                   |
                                          [ Nairobi Hub ]
                                             /        \
                             [ Nairobi Python ]    [ Lagos Vision ]
                                     |                    |
                            [ Jupyter Notebook ] <-> [ Visuelle Ausgabe ]
```

## Installation

### Voraussetzungen
- **Betriebssystem**: Linux oder WSL2 (Kernel 5.10+ für `io_uring` und `memfd` erforderlich).
- **Rust**: 1.70+
- **Python**: 3.10+
- **Systembibliotheken**:
    ```bash
    sudo apt-get update && sudo apt-get install -y \
        build-essential \
        pkg-config \
        libdbus-1-dev \
        python3-dev \
        dbus-x11 \
        libosmesa6-dev \
        mesa-utils
    ```

### Aus dem Quellcode erstellen
1. **Repository klonen**:
    ```bash
    git clone https://github.com/KevinKenya/nairobi-connector-open-source
    cd nairobi-connector-open-source
    ```

2. **Virtuelle Umgebung einrichten**:
    ```bash
    python3 -m venv .venv
    source .venv/bin/activate
    pip install maturin pyo3-build-config zbus anywidget traitlets
    ```

3. **Gesamten Stack bauen**:
    ```bash
    ./build_wheel.sh
    ```

4. **Wheel installieren**:
    ```bash
    pip install target/wheels/nairobi_os-0.3.1-py3-none-any.whl
    ```

## Systemkonfiguration (Leitfaden für Mitwirkende)

### Huge Pages
Die Refinery-Engine priorisiert 1GB Huge Pages für Zero-Copy-Puffer. Um diese auf Ihrem Host zu aktivieren:
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*Hinweis: Wenn 1GB-Seiten nicht verfügbar sind, greift die Engine automatisch auf Transparent Huge Pages (THP) zurück.*

### io_uring und SQPOLL
Die `DiracEngine` verwendet `io_uring` mit `SQPOLL` für maximalen E/A-Durchsatz. `SQPOLL` erfordert in der Regel erhöhte Berechtigungen (`CAP_SYS_ADMIN`) oder einen Kernel, der mit `IORING_SETUP_SQPOLL` konfiguriert ist. Wenn die Engine `SQPOLL` nicht initialisieren kann, fällt sie auf den Standard-`io_uring`-Modus zurück.

## Verwendung
```python
import nairobi_os

# Refinery zünden
nairobi_os.connect()

# Daten in einen SovereignFrame aufnehmen
df = nairobi_os.read_csv("dataset.csv")

# Vektorisierte Analytik durchführen
print(f"Mittelwert: {df.column_name.mean()}")

# Interaktive Visualisierung starten
df.plot()
```

## Testen
Nairobi OS enthält eine umfassende Testsuite, die Rust-Units, IPC-Integration und Python-Bindings abdeckt.

### Alle Tests ausführen
```bash
# Rust-Tests ausführen
cargo test --workspace

# Python-Integrationstests ausführen
python3 test_nairobi.py
```

### Benchmarking
Detaillierte Performance-Benchmarks können im Verzeichnis `nairobi-benchmarks` ausgeführt werden:
```bash
cd nairobi-benchmarks
pip install -r requirements.txt
python orchestration/benchmark_runner.py --workload workloads/workload_nba_pipeline.yaml
```

## Fehlerbehebung
- **D-Bus-Verbindung abgelehnt**: Stellen Sie sicher, dass der `dbus-daemon` läuft. Verwenden Sie in Headless-Umgebungen `dbus-launch`.
- **Lagos-Rendering-Probleme**: Lagos erfordert einen gültigen GPU-Treiber oder OSMesa für Software-Fallback. Überprüfen Sie dies mit `glxinfo`.
- **Huge Page-Zuweisung fehlgeschlagen**: Überprüfen Sie `/proc/meminfo`, um sicherzustellen, dass genügend Huge Pages vom Kernel reserviert wurden.

## Lizenz
Dieses Projekt ist unter der **Apache License 2.0** lizenziert. Lizenziert unter der Apache-Lizenz, Version 2.0.

---
© 2026 Kevin Chege. Alle Rechte vorbehalten.
