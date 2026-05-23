[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Axum Refinery

## Überblick
Die Axum Refinery ist der Hochleistungskern von Nairobi OS. In Rust geschrieben, ist sie darauf ausgelegt, moderne Hardware durch Kernel-Bypass-E/A und vektorisierte parallele Analysen zu sättigen. Sie fungiert als D-Bus-Dienst, der den Lebenszyklus von Daten verwaltet, die in anonyme Speicher-Dateideskriptoren (`memfd`) aufgenommen wurden.

## Hauptmerkmale
- **Dirac Ingestion Engine**: Eine 3-stufige Ingestionsstrategie unter Verwendung von `io_uring` (Stufe 1), `copy_file_range` (Stufe 2) und `mmap` (Stufe 3).
- **Axiom Crunch**: Vektorisierte statistische Momentberechnung (Mittelwert, Varianz, Schiefe, Kurtosis), angetrieben durch Polars und Rayon.
- **Relational Strike**: Optimierte Berechnung der Pearson- und Spearman-Korrelation.
- **SQL-Analytik**: Direkte Ausführung von SQL-Abfragen auf speicherresidenten Daten mit `polars-sql`.
- **Zero-Copy Data Plane**: Exponiert Analyseergebnisse über `iceoryx2` Shared Memory und D-Bus.

## Architektur
Die Refinery ist in spezialisierte Engines unterteilt:
- `DiracEngine`: Verarbeitet hardwarebeschleunigte E/A.
- `AnalyzeEngine`: Führt statistische Berechnungen und SQL-Ausführungen durch.
- `DbusService`: Implementiert die Schnittstelle `org.nairobi.NairobiAxumRefinery1`.

## Installation

### Voraussetzungen
- **Kernel**: Linux 5.10+ (WSL2 unterstützt).
- **Abhängigkeiten**: `libdbus-1-dev`, `pkg-config`.
- **Huge Pages**: Die Engine erbringt die beste Leistung mit aktivierten 1GB Huge Pages.
    ```bash
    echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
    ```

### Bauen
```bash
cargo build --release -p nairobi-axum-refinery
```

## Entwicklung

### Kernel-Level-Konfiguration
Mitwirkende sollten beachten, dass die `DiracEngine` versucht, `IORING_SETUP_SQPOLL` zu verwenden. Damit dies ohne Root funktioniert, müssen Sie möglicherweise `/proc/sys/kernel/unprivileged_userns_clone` anpassen oder mit `CAP_SYS_ADMIN` ausführen.

### Tutorial: Hinzufügen einer neuen statistischen Metrik
1.  **Metrik definieren**: Aktualisieren Sie in `src/analyze.rs` die Struktur `StatisticalProfile` und ihre Methode `compute`.
2.  **Protokoll aktualisieren**: Fügen Sie das neue Feld der Struktur `DistilledAnalytics` in `crates/nairobi-protocol/src/types.rs` hinzu.
3.  **Export über D-Bus**: Stellen Sie sicher, dass die D-Bus-Schnittstelle in `src/dbus_service.rs` das aktualisierte Profil korrekt serialisiert.

## Testen
Die Refinery verwendet `tokio::test` für asynchrone Integrationstests.
```bash
cargo test -p nairobi-axum-refinery
```

#### Mocking für isolierte Tests
Sie können die `AnalyzeEngine` isoliert testen, indem Sie manuell ein `memfd` erstellen und es an die Engine übergeben, wobei die D-Bus-Schicht umgangen wird:
```rust
let opts = memfd::MemfdOptions::default();
let mfd = opts.create("test.csv")?;
// Testdaten schreiben...
let engine = AnalyzeEngine::new()?;
let results = engine.analyze(mfd.into_fd(), "target_column")?;
```

## Fehlerbehebung
- **`io_uring`-Initialisierung fehlgeschlagen**: Prüfen Sie, ob Ihr Kernel `io_uring` unterstützt (`zgrep CONFIG_IO_URING /proc/config.gz`).
- **Huge Page-Zuweisung fehlgeschlagen**: Stellen Sie sicher, dass der Host über genügend zusammenhängenden Speicher verfügt. Prüfen Sie `grep Huge /proc/meminfo`.

## Unterstützung
Wenn Sie Nairobi OS nützlich finden, ziehen Sie bitte in Erwägung, das Projekt zu unterstützen:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Lizenz
Dieses Projekt ist unter der **Apache License 2.0** lizenziert.

---
© 2026 Kevin Chege. Alle Rechte vorbehalten.
