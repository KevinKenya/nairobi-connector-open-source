[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Protocolo Nairobi (Nairobi Protocol)

## Resumen
El crate Nairobi Protocol define las interfaces compartidas de D-Bus, las firmas de GVariant y las estructuras de datos utilizadas en todo el ecosistema de Nairobi OS. Sirve como la "fuente de verdad" para la seguridad de tipos en el núcleo Rust, el orquestador Hub y los enlaces de Python.

## Componentes Clave
- **Definiciones de Interfaz**: Constantes para nombres de servicios, rutas de objetos y firmas de métodos.
- **Tipos Compartidos**: Estructuras compatibles con GVariant como `DistilledAnalytics` y `CorrelationResult`.
- **Gestión de Memoria**: El envoltorio `MemoryPipe` para operaciones `memfd` y las definiciones de arena `iceoryx2`.

## Interfaz D-Bus
- **Nombre del Servicio**: `org.nairobi.NairobiAxumRefinery1`
- **Ruta del Objeto**: `/org/nairobi/NairobiAxumRefinery1`
- **Interfaz**: `org.nairobi.NairobiAxumRefinery1`

## Uso
Agregue este crate como una dependencia en cualquier componente que necesite comunicarse dentro del ecosistema de Nairobi OS.

## Desarrollo
Los cambios en este crate deben realizarse con extremo cuidado, ya que requieren la recompilación de todos los crates dependientes y pueden romper la compatibilidad binaria entre la refinería y los enlaces de Python.

## Pruebas
Las pruebas de integración aseguran que las firmas de GVariant coincidan con el protocolo D-Bus esperado:
```bash
cargo test -p nairobi-protocol
```

## Soporte
Si encuentras útil Nairobi OS, considera apoyar el proyecto:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Licencia
Este proyecto está licenciado bajo la **Apache License 2.0**.

---
© 2026 Kevin Chege. Todos los derechos reservados.
