[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi Hub

## Überblick
Nairobi Hub ist der zentrale IPC (Inter-Process Communication) Orchestrator von Nairobi OS. Er verwaltet die Koordination von Dateideskriptoren, D-Bus-Signalen und Shared-Memory-Segmenten zwischen der Rust-Hochleistungs-Refinery und ihren Clients.

## Hauptmerkmale
- **FD Proxying**: Übergibt `memfd`-Dateideskriptoren sicher über D-Bus unter Verwendung von GVariant-Signaturen.
- **Service Management**: Überwacht und verwaltet den Lebenszyklus von `org.nairobi.NairobiAxumRefinery1`.
- **Hybrid Data Plane**: Routet Daten dynamisch über `iceoryx2` Shared Memory (für Performance) oder D-Bus (für Kompatibilität).
- **Semantische Dekodierung**: Dekodiert rohe binäre Analysen in menschenlesbare Berichte und native Python-Strukturen.

## Architektur
Der Hub ist in mehrere interne Module unterteilt:
- `client.rs`: Der D-Bus-Proxy-Client.
- `shm_subscriber.rs`: Behandelt `iceoryx2` Shared-Memory-Abonnements.
- `decoder.rs`: Wandelt GVariant-Ergebnisse in Markdown und JSON um.

## Verwendung
Der Hub wird primär als Bibliothek von `nairobi-python` verwendet, um mit der Refinery zu kommunizieren.

## Entwicklung
Stellen Sie bei Änderungen am Hub sicher, dass alle Änderungen an der D-Bus-Schnittstelle auch in `nairobi-protocol` reflektiert werden.

## Testen
Integrationstests für den Hub verifizieren den vollständigen IPC-Roundtrip:
```bash
cargo test -p nairobi-hub
```

## Lizenz
Dieses Projekt ist unter der **PolyForm Noncommercial License 1.0.0** lizenziert.

---
© 2026 Kevin Chege. Alle Rechte vorbehalten.
