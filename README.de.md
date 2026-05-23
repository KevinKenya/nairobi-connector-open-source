# Nairobi OS: Hochleistungs-, Zero-Copy-KI & Data-Science-Infrastruktur

[![PyPI Version](https://img.shields.io/pypi/v/nairobi-os.svg)](https://pypi.org/project/nairobi-os/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
![System](https://img.shields.io/badge/Kernel-Linux_6.17_Native-orange.svg)
![Arch](https://img.shields.io/badge/Architecture-x86__64_/_ARM64-red.svg)

---

## Der Ursprung: Aus dem Schmelztiegel zum Metall

Nairobi OS ist nicht das Produkt eines komfortablen Unternehmens-Inkubators oder eines von Risikokapital finanzierten Forschungslabors. Es ist das Ergebnis absoluter Notwendigkeit, geboren aus einer Reihe tiefer persönlicher Krisen und dem unerbittlichen Drang zu handeln, wo Standard-Werkzeuge der Industrie versagen.

Ich bin Kevin Chege, Gründer des Sovereign Systems Lab (Nairobi, Kenia). Von 2009 bis 2022 war mein Leben von schwerem Alkoholismus geprägt. Er kostete mich mein berufliches Ansehen, Chancen und fast mein Leben. Auf dem Höhepunkt meiner Sucht arbeitete ich als Analyst im Strategiebüro der Open University in Milton Keynes, Großbritannien, nachdem ich zuvor als Gründer und Präsident von AIESEC in Ruanda (2006–2010) tätig war. Heute befinde ich mich in meinem vierten Jahr kontinuierlicher Nüchternheit.

```
                     LEGIO XIII GEMINA
              "Die 13. Legion — 13. Juni"
     Dreizehn verlorene Jahre. Dreizehn Jahre zum Zurückfordern.
```

Mein Programmierweg ist in der hardwarenahen Systemarchitektur und extremen Optimierung verwurzelt. Im Jahr 2015 legte ich meine Vision für den Aufbau dezentraler, hochtechnischer Kapazitäten auf dem afrikanischen Kontinent in [dieser Abhandlung über Kenias Silicon Valley](https://www.linkedin.com/pulse/building-kenyas-silicon-valley-making-work-kevin-chege/) dar. Als der LLM-Goldrausch 2023 begann, war ich früh dabei. Ich baute und implementierte LLM-Wrapper, erkannte jedoch schnell deren Grenzen, wie in dieser frühen [LLM-Wrapper-Demonstration aus dem Jahr 2023](https://www.linkedin.com/feed/update/urn:li:activity:7102930955807449088/) dokumentiert.

Mir wurde klar, dass der Aufbau von High-Level-Wrappern auf instabilen APIs eine architektonische Sackgasse war. Der wahre Krieg wird an der Schnittstelle zwischen lokalen Hardwarebeschränkungen und Ressourcenzuweisung geführt.

Das gesamte Jahr 2025 über lebte ich auf einem Lenovo X13 ThinkPad mit einem stark eingeschränkten Hardwareprofil:

```
Prozessor: AMD Ryzen 5 PRO 4650U (6 Kerne, 12 Threads)
Grafik: AMD Radeon RX Vega 6 iGPU
Arbeitsspeicher: 32 GB RAM (mit sehr hoher Systemauslastung)
Festplatte: 256 GB NVMe (zu 99 % voll)
```

Auf genau dieser Maschine entwickelte ich 2025 **Tumz** ([Sarafakai](http://www.sarafakai.com)), eine luftspaltgesicherte (air-gapped), latenzfreie KI zur Unterstützung klinischer Entscheidungen. Sie führte gleichzeitig Live-Audio-Transkriptionen und klinische Inferenzen in Echtzeit auf der integrierten GPU (iGPU) aus, während das gesamte Unified Medical Language System (UMLS) im RAM verblieb. Wir arbeiten derzeit mit einem kenianischen Krankenhaus zusammen, um Tumz im Rahmen einer einjährigen klinischen Studie zu testen – weil die menschliche Gesundheit strenge, empirische Validierung erfordert und keine Vermutungen von Entwicklern.

Während der Entwicklung von Tumz stieß ich auf die massiven, systemischen Ineffizienzen des modernen Data-Science-Stacks:
1. **Die Python-Steuer**: Durchgängiges Kopieren von Speicher, GIL-Engpässe und massiver Runtime-Overhead.
2. **Die Browser-Steuer**: Manifest-V3-Komplikationen, Rendering-Latenz und hochfrequente Kommunikationsfehler in lang laufenden Agenten-Konversationen.
3. **Der OS-Kernel-Engpass**: Ineffizientes Prozess-Scheduling, CPU-Thread-Starvation und Display-Server-Overhead (Wayland vs. X11 Kontextwechsel).

Deshalb habe ich mich Ende 2025 daran gemacht, einen Infrastruktur-Stack zu entwickeln, der diese Grenzen vollständig umgeht – ein Agentic Operating System, das für Zero-Copy-Datenpipelines und hardwarenahe KI-Ausführung ausgelegt ist. Dieses Repository ist der Open-Source-Kern dieses Systems.

---

## Globale Akzeptanz & Telemetrie

Nairobi OS wurde am 6. Mai 2026 veröffentlicht und hat bei Systemprogrammierern, quantitativen Forschern und Edge-Computing-Architekten weltweit rasch an Beliebtheit gewonnen.

### Kumulierte globale Verteilung (6. Mai 2026 – 23. Mai 2026)

| Metrik | Messung | Kontext |
| :--- | :--- | :--- |
| **Globaler Rang** | **#75.293** | Aus 797.894 aktiven Paketen auf PyPI |
| **Perzentil** | **9,43 %** | Spitzenplatzierung für Python-Erweiterungen auf Systemebene |
| **Downloads Gesamt** | **1.525** | Saubere, organische Downloads von Entwicklern mit hoher Kaufabsicht |

### Download-Volumen nach Version

```
  0.2.0 [████████████████████████████████████████] 342
  0.2.1 [██████████████████████████] 224
  0.3.0 [████████████████████████] 212
  0.3.1 [████████████████████] 176
  0.1.0 [███████████████████] 169
  0.4.1 [██████████████] 120
```

### Top 10 der souveränen Regionen bei der Einführung

| Rang | Region | Ländercode | Download-Volumen |
| :--- | :--- | :--- | :--- |
| 1 | Vereinigte Staaten | US | 661 |
| 2 | Hongkong | HK | 103 |
| 3 | China | CN | 84 |
| 4 | Deutschland | DE | 74 |
| 5 | Japan | JP | 65 |
| 6 | Singapur | SG | 56 |
| 7 | Vereinigtes Königreich | GB | 51 |
| 8 | Frankreich | FR | 51 |
| 9 | Russland | RU | 42 |
| 10 | Südkorea | KR | 30 |

---

## Unterstützung & Souveränität

Wenn Nairobi OS Ihre Datenpipelines optimiert, Ihre Cloud-Kosten senkt oder Ihre lokalen Agenten-Architekturen antreibt, ziehen Sie bitte in Erwägung, unsere unabhängige Systemforschung zu unterstützen. Jeder Beitrag fließt direkt in Compiler-Optimierungen auf Hardwareebene und Edge-Compute-Tests in Nairobi.

[![Nairobi OS Entwicklung unterstützen](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

---

## Sprachoptionen

[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

---

## Hauptfunktionen

* **Computernutzung ohne Pixel**: Umgeht langsame, teure visuelle Agenten-Pipelines. Interagiert nativ mit dem Linux-Desktop über AT-SPI2 und den TOON (Token-Oriented Object Notation)-Komprimierungsalgorithmus und speist hierarchische Bäume direkt in LLMs ein.
* **Zero-Copy-Ingestion**: Hardwarebeschleunigtes Datenladen unter Umgehung des Kernels mittels `io_uring` und 1GB Huge Pages.
* **Hardwarebeschleunigte Visualisierung**: Latenzarmes, interaktives Jupyter-Plotting über den `lagos-lite` Rendering-Daemon, basierend auf `wgpu` und `egui`.
* **Vektorisierte analytische Ausführung**: Extreme CPU-Sättigung durch Polars-Abfrageausführung und Rayon-Datenpipelines mit Multithreading.
* **Sovereign Interface**: Eine flüssige Python-API (`SovereignFrame`), die im Arbeitsspeicher abgebildete Dateideskriptoren und IPC kapselt.

---

## Open Source vs. Enterprise-Architektur

Nairobi OS ist strukturell zweigeteilt. Das Open-Source-Repository bietet grundlegende Hochleistungs-Datenverarbeitung und Einzelknoten-Visualisierungsprimitiven. Das closed-source kommerzielle Ökosystem enthält fortschrittliche Multi-Agenten-, Hochverfügbarkeits- und branchenspezifische Implementierungen.

```
                                  +---------------------------------------+
                                  |         Nairobi Python API            |
                                  +---------------------------------------+
                                                      |
                                     [ GVariant über D-Bus / Shared Memory ]
                                                      |
                                                      v
                                  +---------------------------------------+
                                  |           Nairobi Hub                 |
                                  +---------------------------------------+
                                                      |
                    +---------------------------------+---------------------------------+
                    |                                                                   |
                    v                                                                   v
     +------------------------------+                                    +------------------------------+
     |     Axum Refinery (Daten)    | <===[ Zero-Copy IPC / iceoryx2 ]==>|     Lagos Vision (Visuell)   |
     +------------------------------+                                    +------------------------------+
```

### Open-Source-Crate-Workspace (`crates/`)

1. **`nairobi-axum-refinery`**: Leistungsstarker Rust-Daemon zur Verwaltung der Rohdatenaufnahme, parallelisierter Statistiken über Rayon und vektorisierter Abfrageausführung über Polars.
2. **`nairobi-hub`**: Der zentrale IPC-Orchestrator. Verwaltet und leitet Dateideskriptoren und Signale zwischen Clients und dem Refinery-Daemon weiter.
3. **`lagos-lite`**: Der visuelle Kortex. Eine Headless, ereignisgesteuerte Rendering-Engine, die im Arbeitsspeicher abgebildete Dateien direkt in die GPU-Pipeline einbindet.
4. **`nairobi-protocol`**: Die gemeinsame Protokollschicht. Definiert Standard-GVariant-Serialisierungsschemata, Fehlertypen und Shared-Memory-Layouts.
5. **`nairobi-python`**: Das Python-Erweiterungsmodul, kompiliert über `PyO3` und paketiert mit `Maturin`.

### Privates Unternehmens-Ökosystem (`modules/`)

Unsere Enterprise-Komponenten befinden sich in einem privaten Repository (`Sovereign-Systems-Lab`) und sind für industrielle, finanzielle und staatliche Infrastrukturen lizenziert.

1. **`sovereign-ui`**: Die Enterprise AT-SPI2 Engine. Implementiert Aegis-Protokoll-Sicherheit, Hardwarebindung und Desktop-Manipulation auf Produktionsniveau.
2. **`nairobi-connector`**: Fortgeschrittener Model Context Protocol (MCP) Server, der rohe, latenzarme D-Bus-Signale für Enterprise LLMs verwaltet.
3. **`tactical-rtos-node`**: Latenzfreier Echtzeit-Betriebssystem-Scheduler für sicherheitskritische industrielle Edge-Automatisierung.
4. **`industrial-guardian-rust` / `industrial-guardian-python`**: Autonome SRE-Schicht (Site Reliability Engineering) mit präventiver Vermeidung von OOM, Speicherlecks und Systemabstürzen.
5. **`fintech-bridge-rust`**: Echtzeit-Hochfrequenz-Transaktionsparser und Brücke zu Legacy-Mainframes (EBCDIC/SBA Terminal-Parsing).
6. **`aviation-audio-rust`**: Sub-Millisekunden-loses, sperrfreies Audio-Stream-Processing, akustische Telemetrieanalyse und rohe Wellen-DSP.
7. **`drawbridge_api`**: Sichere, authentifizierte Multi-Tenant-gRPC-Zugbrücke, die den lokalen Kernel von nicht vertrauenswürdigen Cloud-Agent-Aufrufen isoliert.

### Matrix zum Leistungsvergleich

| Fähigkeit / Funktion | Open Source Core (`crates/`) | Enterprise Suite (`modules/`) |
| :--- | :---: | :---: |
| **Ingestion Engine** | `mmap` / `copy_file_range` | `io_uring` + `SQPOLL` + 1GB Huge Pages |
| **Statistische Analyse** | Einfache deskriptive Statistik | Vektorisierte, mehrteilige Schiefe/Kurtosis, Korrelation |
| **Query Engine** | In-Process Polars SQL | Verteiltes Apache Arrow / DataFusion Cluster |
| **IPC-Mechanismus** | POSIX Shared Memory / D-Bus | Zero-Copy `iceoryx2` Shared-Memory-Arenen |
| **Visualisierung** | Lokales Jupyter `anywidget` | WebRTC GStreamer / transparente Wayland Layer-Shell Overlays |
| **Sicherheit & Compliance** | Standard POSIX-Grenzen | Aegis-Protokoll, SHA-256 verkettetes forensisches Ledger |
| **Authentifizierung** | Keine (Lokaler vertrauenswürdiger Benutzer) | Hardwarebindung (TPM 2.0 / CPU ID), private PKI |
| **Plattform-Ziel** | Einzelknoten-Linux | Verteilte Cloud / Edge-Knoten / Hochfrequenzhandel |

---

## Installation & Einrichtung

### Voraussetzungen
- **OS**: Linux (Ubuntu 22.04+ empfohlen) oder Windows Subsystem für Linux (WSL2).
- **GPU**: Vulkan-, Metal- oder OpenGL-kompatibler Treiber.
- **Python**: 3.10 oder neuer.
- **Rust**: Stabile Toolchain (falls aus dem Quellcode gebaut wird).

### Schnellinstallation (PyPI)
```bash
pip install nairobi-os
```

### Aus dem Quellcode kompilieren
Um den gesamten Workspace zu kompilieren, einschließlich der nativen Daemons und der Python-Erweiterung:

1. **Repository klonen**:
   ```bash
   git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
   cd nairobi-connector-open-source
   ```

2. **Virtuelle Umgebung konfigurieren**:
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   pip install maturin pyo3-build-config zbus anywidget traitlets pandas
   ```

3. **Workspace-Build ausführen**:
   ```bash
   chmod +x build_wheel.sh
   ./build_wheel.sh --release
   ```
   Dies kompiliert die nativen Daemons, kopiert sie in das Paketverzeichnis und erstellt ein Wheel unter `crates/nairobi-python/target/wheels/`.

---

## Nutzungsleitfaden

### 1. Datenanalyse (Die In-Memory-Pipeline)

Nairobi OS bietet die `SovereignFrame` API. Sie verarbeitet Speicherabbildungen im Hintergrund und ermöglicht so eine schnelle Datenmanipulation.

```python
import nairobi_os as nb

# Starten des Refinery-Daemons im Hintergrund
nb.connect()

# Einlesen des Datensatzes über eine Zero-Copy-Speicher-Pipeline
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# Ausführen vektorisierter Berechnungen über die Rust-Refinery
profile = frame.crunch("value")
print(f"Mittelwert: {profile['mean']:.4f}")
print(f"Standardabweichung: {profile['std_dev']:.4f}")

# Ausführen beliebiger SQL-Abfragen direkt auf dem im Speicher abgebildeten Frame
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# Starten des Lagos-beschleunigten interaktiven Plotting-Widgets
subset.plot(column="value")
```

### 2. Computernutzung ohne Pixel (MCP)

Um die semantische AT-SPI2-Schnittstelle zu nutzen, sollte Ihr KI-Agent mit den bereitgestellten MCP-Server-Tools interagieren, anstatt Screenshots auszuwerten:

```
                    ABLAUF DER COMPUTERNUTZUNG
                     
  [ LLM Agent ]                                 [ Nairobi OS ]
        |                                             |
        |===> nairobi_find_window("Text Editor") ====>| (Lokalisiert das Ziel)
        |<=== Gibt Fenster-ID & Grenzen zurück =======|
        |                                             |
        |===> nairobi_get_ui_map() ==================>| (Generiert TOON)
        |<=== Gibt komprimierten Markdown-Baum zurück |
        |     "[ID: 12] Button: 'Save'"               |
        |                                             |
        |===> nairobi_interact(12, "click") =========>| (Führt Aktion aus)
        |<=== Gibt Erfolgsstatus zurück =============|
```

---

## System-Tuning (Leitfaden für Mitwirkende)

Um die in unseren Benchmarks gezeigten Leistungsprofile zu erreichen, muss Ihr Host-Kernel für System-Level-Speicherabbildung konfiguriert sein.

### 1GB Huge Pages
Nairobi OS verwendet 1GB Huge Pages, um den Übersetzungs-Overhead des Translation Lookaside Buffer (TLB) der CPU bei massiven Datensätzen zu umgehen.

So weisen Sie eine Huge Page auf Ihrem Linux-Host zu:
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*Hinweis: Wenn das System aufgrund von Fragmentierung keine 1GB-Seite zuweisen kann, weicht die Engine automatisch auf Transparent Huge Pages (THP) aus.*

### D-Bus-Broker-Konfiguration
Stellen Sie in hochfrequenten Umgebungen sicher, dass `dbus-broker` anstelle des alten `dbus-daemon` installiert ist, um eine schnelle Signalweiterleitung über die Steuerungsebene zu gewährleisten.

---

## Lizenz

Dieses Projekt ist unter der **Apache License 2.0** lizenziert.  
*(Hinweis: Teile des TOON-Formats und der Brückenimplementierung werden den TOON-Autoren zugeschrieben).*

---
© 2026 Kevin Chege. Alle Rechte vorbehalten.  
*Sovereign Systems Lab, Nairobi, Kenia.*
