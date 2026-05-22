[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi Hub

## Resumen
Nairobi Hub es el orquestador central de IPC (Comunicación entre Procesos) de Nairobi OS. Gestiona la coordinación de descriptores de archivos, señales de D-Bus y segmentos de memoria compartida entre la refinería Rust de alto rendimiento y sus clientes.

## Características Clave
- **Proxy de FD**: Pasa de forma segura descriptores de archivos `memfd` a través de D-Bus utilizando firmas de GVariant.
- **Gestión de Servicios**: Supervisa y gestiona el ciclo de vida de `org.nairobi.NairobiAxumRefinery1`.
- **Plano de Datos Híbrido**: Enruta dinámicamente los datos a través de la memoria compartida `iceoryx2` (por rendimiento) o D-Bus (por compatibilidad).
- **Decodificación Semántica**: Decodifica analíticas binarias brutas en informes legibles por humanos y estructuras nativas de Python.

## Arquitectura
El Hub se divide en varios módulos internos:
- `client.rs`: El cliente proxy de D-Bus.
- `shm_subscriber.rs`: Gestiona las suscripciones a la memoria compartida de `iceoryx2`.
- `decoder.rs`: Convierte los resultados de GVariant en Markdown y JSON.

## Uso
El Hub es utilizado principalmente como una biblioteca por `nairobi-python` para comunicarse con la refinería.

## Desarrollo
Al modificar el Hub, asegúrese de que cualquier cambio en la interfaz de D-Bus también se refleje en `nairobi-protocol`.

## Pruebas
Las pruebas de integración para el Hub verifican el viaje de ida y vuelta completo de IPC:
```bash
cargo test -p nairobi-hub
```

## Soporte
Si encuentras útil Nairobi OS, considera apoyar el proyecto:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Licencia
Este proyecto está licenciado bajo la **Apache License 2.0**.

---
© 2026 Kevin Chege. Todos los derechos reservados.
