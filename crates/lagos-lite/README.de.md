[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Lagos Vision (lagos-lite)

## Überblick
Lagos Vision ist die Hochleistungs-Rendering-Engine für Nairobi OS. Sie ist darauf ausgelegt, Millionen von Datenpunkten mit einer Latenz von weniger als einer Millisekunde zu visualisieren, indem analytische Daten direkt in die GPU-Pipeline gemappt werden. Lagos arbeitet als Headless-Daemon, der JPEG-kodierte Frames über WebSockets an Jupyter Notebook Widgets streamt.

## Hauptmerkmale
- **Zero-Copy Rendering**: Daten werden von `memfd`-Handles direkt in `wgpu`-Puffer gemappt.
- **Hardwarebeschleunigung**: Verwendet `egui` und `wgpu` (Vulkan, Metal, DX12 oder OpenGL) für Hochleistungs-Plotting.
- **LTTB-Downsampling**: Implementiert den Largest-Triangle-Three-Buckets-Algorithmus auf der GPU, um die visuelle Genauigkeit beim Rendering massiver Datensätze beizubehalten.
- **Ereignisgesteuerte Architektur**: Verbraucht im Leerlauf keine CPU; rendert nur bei Datenaktualisierungen oder Benutzerinteraktion.

## Architektur
Lagos Vision besteht aus:
- **Lagos Lite**: Die Kernbibliothek, die die Rendering-Pipeline bereitstellt.
- **Lagos Vision Daemon**: Der binäre Prozess, der die `wgpu`-Oberfläche und den WebSocket-Server verwaltet.
- **Lagos Widget**: Eine `anywidget` Python-Komponente, die den Stream anzeigt.

## Installation

### Voraussetzungen
- **GPU**: Eine Vulkan-kompatible GPU (oder OSMesa für Software-Fallback).
- **Systembibliotheken**: `libosmesa6-dev`, `mesa-utils`, `xvfb`.

### Bauen
```bash
cargo build --release -p lagos-lite --bin lagos-vision-daemon
```

## Verwendung

### In Nairobi OS
Lagos wird normalerweise über die Methode `SovereignFrame.plot()` in Python verwendet.

### Manuelles Debuggen
Sie können den Daemon manuell starten, um die Rendering-Pipeline zu testen:
```bash
./target/release/lagos-vision-daemon --fd <FD_INT> --width 1000 --height 400
```

## Entwicklung

### Implementierung einer benutzerdefinierten Visualisierungsebene
1.  **Pipeline modifizieren**: Definieren Sie in `src/pipeline.rs` Ihre Vertex- und Fragment-Shader (WGSL).
2.  **Puffer-Layout aktualisieren**: Mappen Sie die eingehenden `memfd`-Daten auf die Bindegruppen Ihres neuen Shaders.
3.  **UI-Integration**: Fügen Sie Steuerelemente (Schieberegler, Schaltflächen) zur `egui`-Schnittstelle in `src/device.rs` hinzu.

### Headless-Umgebungen
In Umgebungen wie Google Colab verwendet Lagos `xvfb-run` oder OSMesa, um das Fehlen eines physischen Displays zu handhaben:
```bash
xvfb-run -s "-screen 0 1024x768x24" ./target/release/lagos-vision-daemon ...
```

## Testen
Lagos enthält visuelle Integrationstests, die Frames erfassen und sie mit Golden Images vergleichen.
```bash
cargo test -p lagos-lite
```

## Fehlerbehebung
- **WebSocket-Verbindung fehlgeschlagen**: Stellen Sie in Cloud-Umgebungen (Colab/SageMaker) sicher, dass der Proxy-Port korrekt gemappt ist. Nairobi Python erledigt dies automatisch, wenn `google.colab` erkannt wird.
- **WGPU-Adapter nicht gefunden**: Stellen Sie sicher, dass GPU-Treiber installiert sind. Wenn Sie eine reine CPU-Umgebung verwenden, versucht Lagos, auf einen Software-Adapter auszuweichen.

## Unterstützung
Wenn Sie Nairobi OS nützlich finden, ziehen Sie bitte in Erwägung, das Projekt zu unterstützen:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Lizenz
Dieses Projekt ist unter der **Apache License 2.0** lizenziert.

---
© 2026 Kevin Chege. Alle Rechte vorbehalten.
