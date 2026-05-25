[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md) | [Finnish](README.fi.md) | [Cantonese](README.yue.md) | [Français](README.fr.md) | [Nederlands](README.nl.md)

# Nairobi OS: Hoogwaardige, Zero-Copy AI & Data Science Infrastructuur

[![PyPI Version](https://img.shields.io/pypi/v/nairobi-os.svg)](https://pypi.org/project/nairobi-os/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
![System](https://img.shields.io/badge/Kernel-Linux_6.17_Native-orange.svg)
![Arch](https://img.shields.io/badge/Architecture-x86__64_/_ARM64-red.svg)

---

## De Oorsprong: Van de Smeltkroes naar het Metaal

Nairobi OS is niet het product van een comfortabele bedrijfsincubator of een door durfkapitaal gefinancierd onderzoekslab. Het is het resultaat van absolute noodzaak, geboren uit een reeks diepe persoonlijke crises en een meedogenloze drang om te presteren waar standaard industriële tools falen.

Ik ben Kevin Chege, oprichter van Sovereign Systems Lab (Nairobi, Kenia). Van 2009 tot 2022 werd mijn leven beheerst door ernstig alcoholisme. Het kostte me mijn professionele status, kansen en bijna mijn leven. Op het hoogtepunt van mijn verslaving werkte ik als analist op het strategiebureau van The Open University in Milton Keynes (VK), na mijn tijd als oprichter en president van AIESEC in Rwanda (2006–2010). Vandaag ben ik in mijn vierde jaar van voortdurende nuchterheid.

```
                     LEGIO XIII GEMINA
              "Het 13e Legioen — 13 juni"
     Dertien verloren jaren. Dertien jaren om terug te eisen.
```

Mijn programmeerreis is geworteld in low-level systeemarchitectuur en extreme optimalisatie. In 2015 legde ik mijn visie vast voor het bouwen van gedecentraliseerde, hoogtechnologische capaciteiten op het Afrikaanse continent in dit traktaat over de Silicon Valley van Kenia. Toen de LLM-goudkoorts in 2023 begon, was ik er vroeg bij. Ik bouwde en implementeerde LLM-wrappers, maar herkende al snel hun beperkingen, zoals gedocumenteerd in deze vroege LLM-wrapper demonstratie uit 2023.

Ik realiseerde me dat het bouwen van high-level wrappers bovenop onstabiele API's een architecturale doodlopende weg was. De echte oorlog wordt uitgevochten op het snijvlak van lokale hardwarebeperkingen en resource-allocatie.

Gedurende 2025 leefde ik op een Lenovo X13 ThinkPad met een zeer beperkt hardwareprofiel:

```
Processor: AMD Ryzen 5 PRO 4650U (6 Cores, 12 Threads)
Grafisch: AMD Radeon RX Vega 6 iGPU
Geheugen: 32 GB RAM (met hoog systeemgebruik)
Opslag: 256 GB NVMe (99% vol)
```

Op exact deze machine heb ik 2025 besteed aan het bouwen van Tumz (Sarafakai), een air-gapped, zero-latency AI voor klinische beslissingsondersteuning. Het voerde gelijktijdig live audio-transcriptie en klinische inferentie uit op de geïntegreerde GPU (iGPU), terwijl het volledige Unified Medical Language System (UMLS) in het RAM-geheugen bleef. We werken momenteel samen met een Keniaans ziekenhuis om Tumz te testen in een klinische proef van een jaar — omdat menselijke gezondheid strikte, empirische validatie vereist, geen aannames van ontwikkelaars.

Tijdens de ontwikkeling van Tumz stuitte ik op de enorme, systemische inefficiënties van de moderne data science stack:
1. **De Python-taks**: Continue geheugenkopieën, GIL-bottlenecks en enorme runtime-overhead.
2. **De Browser-taks**: Manifest V3-complicaties, rendering-latentie en frequente communicatiefouten in langlopende agentische conversaties.
3. **De OS Kernel-bottleneck**: Inefficiënte procesplanning, CPU-thread uithongering en display server overhead (Wayland vs. X11 context switching).

Daarom ben ik eind 2025 begonnen met het bouwen van een infrastructuurstack die deze limieten volledig omzeilt — een Agentisch Besturingssysteem ontworpen voor zero-copy datapijplijnen en hardware-native AI-executie. Deze repository is de open-source kern van die motor.

---

## Wereldwijde Tractie & Télémétrie

Gelanceerd op 6 mei 2026, heeft Nairobi OS snel terrein gewonnen onder systeemprogrammeurs, kwantitatieve onderzoekers en edge computing-architecten wereldwijd. Deze downloadstatistieken zijn afkomstig van het live ClickPy Nairobi OS Dashboard.

### Cumulatieve Wereldwijde Distributie (6 mei 2026 – 23 mei 2026)

| Metriek | Meting | Context |
| :--- | :--- | :--- |
| **Wereldwijde Rang** | **#75.293** | Van de 797.894 actieve pakketten op PyPI |
| **Percentiel** | **9,43%** | Top-tier ranking voor systeem-niveau Python extensies |
| **Totaal Downloads** | **1.525** | Schone, organische downloads van ontwikkelaars |

### Downloadvolume per Versie

```
  0.2.0 [████████████████████████████████████████] 342
  0.2.1 [██████████████████████████] 224
  0.3.0 [████████████████████████] 212
  0.3.1 [████████████████████] 176
  0.1.0 [███████████████████] 169
  0.4.1 [██████████████] 120
```

### Top 10 Soevereine Regio's van Adoptie

| Rang | Regio | Landcode | Downloadvolume |
| :--- | :--- | :--- | :--- |
| 1 | Verenigde Staten | US | 661 |
| 2 | Hong Kong | HK | 103 |
| 3 | China | CN | 84 |
| 4 | Duitsland | DE | 74 |
| 5 | Japan | JP | 65 |
| 6 | Singapore | SG | 56 |
| 7 | Verenigd Koninkrijk | GB | 51 |
| 8 | Frankrijk | FR | 51 |
| 9 | Rusland | RU | 42 |
| 10 | Zuid-Korea | KR | 30 |

---

## Ondersteuning & Souvereiniteit

Als Nairobi OS uw datapijplijnen optimaliseert, uw cloudkosten verlaagt of uw lokale agentische architecturen aandrijft, overweeg dan om ons onafhankelijke systeemonderzoek te steunen. Elke bijdrage wordt direct ingezet voor compiler-optimalisaties op hardwareniveau en edge-compute testen in Nairobi.

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

Voor directe vragen: aiwithafrica@gmail.com

---

## Belangrijkste Kenmerken

* **Computergebruik Zonder Pixels**: Omzeilt trage, dure op visie gebaseerde agent-pijplijnen. Interageert systeemeigen met de Linux-desktop via AT-SPI2 en het TOON (Token-Oriented Object Notation) compressie-algoritme, waarbij ruwe hiërarchische bomen direct naar LLM's worden gestuurd.
* **Zero-Copy Ingestie**: Hardware-versnelde data-inname via kernel-bypass met gebruik van `io_uring` en 1GB Huge Pages.
* **Hardware-versnelde Visualisatie**: Interactieve Jupyter-plots met lage latentie via de lagos-lite rendering daemon, gebouwd op `wgpu` en `egui`.
* **Gevectoriseerde Analytische Executie**: Extreme CPU-verzadiging door gebruik van Polars query-executie en Rayon multi-threaded datapijplijnen.
* **Sovereign Interface**: Een vloeiende Python API (`SovereignFrame`) die geheugen-gemapte bestandsdescriptors en IPC inkapselt.

---

## Open Source vs. Enterprise Architectuur

Nairobi OS is structureel gesplitst. De open-source repository biedt de fundamentele krachtige dataverwerking en visualisatie-primitieven voor één node. Het gesloten commerciële ecosysteem bevat de geavanceerde multi-agent, high-availability en industriespecifieke implementaties.

```
                                  +---------------------------------------+
                                  |         Nairobi Python API            |
                                  +---------------------------------------+
                                                      |
                                     [ GVariant over D-Bus / shared memory ]
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
     |     Axum Refinery (Data)     | <===[ Zero-Copy IPC / iceoryx2 ]==> |     Lagos Vision (Visual)    |
     +------------------------------+                                    +------------------------------+
```

### Open Source Crate Workspace (`crates/`)

1. **`nairobi-axum-refinery`**: Krachtige Rust-daemon die ruwe data-inname, door Rayon geparallelliseerde statistieken en door Polars gevectoriseerde query-executie beheert.
2. **`nairobi-hub`**: De centrale IPC-orchestrator. Beheert en routeert bestandsdescriptors en signalen tussen clients en de refinery-daemon.
3. **`lagos-lite`**: De visuele cortex. Een headless, event-driven rendering engine die geheugen-gemapte bestanden direct in de GPU-pijplijn mapt.
4. **`nairobi-protocol`**: De gedeelde protocollaag. Definieert standaard GVariant-serialisatieschema's, fouttypes en gedeelde geheugenlay-outs.
5. **`nairobi-python`**: De Python-extensiemodule gecompileerd via `PyO3` en verpakt met `Maturin`.

### Privé Bedrijfsecosysteem (`modules/`)

Onze enterprise-tier componenten bevinden zich in een privé-repository (`Sovereign-Systems-Lab`) en zijn gelicentieerd voor industriële, financiële en infrastructurele toepassingen op staatsniveau.

1. **`sovereign-ui`**: De enterprise AT-SPI2 engine. Implementeert het Aegis Protocol voor beveiliging, hardware-binding en desktopmanipulatie van productiekwaliteit.
2. **`nairobi-connector`**: Geavanceerde Model Context Protocol (MCP) server die ruwe, low-latency D-Bus signalen beheert voor enterprise LLM's.
3. **`tactical-rtos-node`**: Ultra-low-latency, real-time besturingssysteem scheduler voor veiligheidskritische industriële automatisering.
4. **`industrial-guardian-rust` / `industrial-guardian-python`**: Autonome site reliability engineering (SRE) laag met voorspellende OOM-, geheugenlek- en systeemcrash-preventie.
5. **`fintech-bridge-rust`**: Real-time high-frequency transactie-parser en brug naar legacy mainframes (EBCDIC/SBA terminal parsing).
6. **`aviation-audio-rust`**: Lock-free audio-verwerking onder de milliseconde, akoestische telemétrie-analyse en ruwe wave DSP.
7. **`drawbridge_api`**: Beveiligde, geauthenticeerde gRPC-valbrug die de lokale kernel isoleert van onbetrouwbare cloud-agent aanroepen.

### Capability Comparatiematrix

| Capaciteit / Functie | Open Source Kern (`crates/`) | Enterprise Suite (`modules/`) |
| :--- | :---: | :---: |
| **Ingestie Engine** | `mmap` / `copy_file_range` | `io_uring` + `SQPOLL` + 1GB Huge Pages |
| **Statistische Analyse** | Basis beschrijvende stats | Gevectoriseerd, multi-pass skew/kurtosis, correlatie |
| **Query Engine** | In-process Polars SQL | Gedistribueerd Apache Arrow / DataFusion cluster |
| **IPC Mechanisme** | POSIX shared memory / D-Bus | Zero-Copy `iceoryx2` shared memory arenas |
| **Visualisatie** | Local Jupyter `anywidget` | WebRTC GStreamer / transparent Wayland overlays |
| **Beveiliging & Compliance** | Standaard POSIX grenzen | Aegis Protocol, SHA-256 Chain Forensic Ledger |
| **Authenticatie** | Geen (Lokale vertrouwde gebruiker) | Hardware Binding (TPM 2.0 / CPU ID), private PKI |
| **Platform Doel** | Single-node Linux | Gedistribueerde Cloud / Edge Node / HF Trading |

---

## Installatie & Configuratie

### Vereisten
- **OS**: Linux (Ubuntu 22.04+ aanbevolen) of Windows Subsystem for Linux (WSL2).
- **GPU**: Vulkan, Metal, of OpenGL compatibele driver.
- **Python**: 3.10 of nieuwer.
- **Rust**: Stabiele toolchain (indien gebouwd vanuit broncode).

### Snel Installeren (PyPI)
```bash
pip install nairobi-os
```

### Bouwen vanuit Broncode
Om de volledige workspace te compileren, inclusief de native daemons en Python-extensie:

1. **Kloon de Repository**:
   ```bash
   git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
   cd nairobi-connector-open-source
   ```

2. **Configureer Virtuele Omgeving**:
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   pip install maturin pyo3-build-config zbus anywidget traitlets pandas
   ```

3. **Voer Workspace Build uit**:
   ```bash
   chmod +x build_wheel.sh
   ./build_wheel.sh --release
   ```
   Dit compileert de native daemons, kopieert ze naar de pakketmap en bouwt een wheel onder `crates/nairobi-python/target/wheels/`.

---

## Gebruikershandleiding

### 1. Data Analytics (De In-Memory Pijplijn)

Nairobi OS biedt de `SovereignFrame` API. Deze regelt ruwe geheugenmapping onder de motorkap, wat snelle datamanipulatie mogelijk maakt.

```python
import nairobi_os as nb

# Start de achtergrond refinery daemon
nb.connect()

# Neem dataset op via zero-copy geheugenpijp
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# Voer gevectoriseerde berekeningen uit via de Rust refinery
profile = frame.crunch("value")
print(f"Gemiddelde: {profile['mean']:.4f}")
print(f"Standaarddeviatie: {profile['std_dev']:.4f}")

# Voer willekeurige SQL-queries direct uit op het geheugen-gemapte frame
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# Start de Lagos-versnelde interactieve plotting widget
subset.plot(column="value")
```

### 2. Computergebruik Zonder Pixels (MCP)

Om de AT-SPI2 semantische interface te gebruiken, moet uw AI-agent communiceren met de beschikbare MCP-servertools in plaats van screenshots te lezen:

```
                     COMPUTERGEBRUIK SEQUENTIE
                     
  [ LLM Agent ]                                 [ Nairobi OS ]
        |                                             |
        |===> nairobi_find_window("Text Editor") ====>| (Lokaliseert doel)
        |<=== Retourneert Window ID & Grenzen ========|
        |                                             |
        |===> nairobi_get_ui_map() ==================>| (Genereert TOON)
        |<=== Retourneert gecomprimeerde Markdown Boom |
        |     "[ID: 12] Button: 'Save'"               |
        |                                             |
        |===> nairobi_interact(12, "click") =========>| (Voert actie uit)
        |<=== Retourneert Successtatus ===============|
```

---

## Systeemoptimalisatie (Gids voor Bijdragers)

Om de prestatieprofielen uit onze benchmarks te behalen, moet uw host-kernel geconfigureerd zijn voor geheugenmapping op systeemniveau.

### 1GB Huge Pages
Nairobi OS gebruikt 1GB Huge Pages om de Translation Lookaside Buffer (TLB) overhead van de CPU bij enorme datasets te omzeilen.

Om een Huge Page toe te wijzen op uw Linux-host:
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*Opmerking: Als het systeem geen pagina van 1GB kan toewijzen vanwege fragmentatie, valt de engine automatisch terug op Transparent Huge Pages (THP).*

### D-Bus Broker Configuratie
In omgevingen met hoge frequentie dient u `dbus-broker` te installeren in plaats van de verouderde `dbus-daemon` om snelle signaalverspreiding over het control plane te garanderen.

---

## Licentie

Dit project is gelicentieerd onder de **Apache License 2.0**.  
*(Opmerking: Delen van het TOON-formaat en de bridge-implementatie worden toegeschreven aan de TOON-auteurs.)*

---
© 2026 Kevin Chege. Alle rechten voorbehouden.  
*Sovereign Systems Lab, Nairobi, Kenia.*