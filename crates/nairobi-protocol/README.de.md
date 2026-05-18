[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi Protocol

## Überblick
Das Nairobi Protocol Crate definiert die gemeinsamen D-Bus-Schnittstellen, GVariant-Signaturen und Datenstrukturen, die im gesamten Nairobi OS-Ökosystem verwendet werden. Es dient als "Source of Truth" für die Typsicherheit im Rust-Kern, dem Hub-Orchestrator und den Python-Bindings.

## Hauptkomponenten
- **Schnittstellendefinitionen**: Konstanten für Dienstnamen, Objektpfade und Methodensignaturen.
- **Gemeinsame Typen**: GVariant-kompatible Strukturen wie `DistilledAnalytics` und `CorrelationResult`.
- **Speicherverwaltung**: Der `MemoryPipe`-Wrapper für `memfd`-Operationen und die `iceoryx2`-Arena-Definitionen.

## D-Bus-Schnittstelle
- **Dienstname**: `org.nairobi.NairobiAxumRefinery1`
- **Objektpfad**: `/org/nairobi/NairobiAxumRefinery1`
- **Schnittstelle**: `org.nairobi.NairobiAxumRefinery1`

## Verwendung
Fügen Sie dieses Crate als Abhängigkeit in jeder Komponente hinzu, die innerhalb des Nairobi OS-Ökosystems kommunizieren muss.

## Entwicklung
Änderungen an diesem Crate sollten mit äußerster Sorgfalt vorgenommen werden, da sie eine Neukompilierung aller abhängigen Crates erfordern und die binäre Kompatibilität zwischen der Refinery und den Python-Bindings beeinträchtigen können.

## Testen
Integrationstests stellen sicher, dass die GVariant-Signaturen mit dem erwarteten D-Bus-Protokoll übereinstimmen:
```bash
cargo test -p nairobi-protocol
```

## Lizenz
Dieses Projekt ist unter der **Apache License 2.0** lizenziert.

---
© 2026 Kevin Chege. Alle Rechte vorbehalten.
