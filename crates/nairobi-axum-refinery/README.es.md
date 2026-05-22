[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi Axum Refinery

## Resumen
La Axum Refinery es el núcleo de alto rendimiento de Nairobi OS. Escrita en Rust, está diseñada para saturar el hardware moderno a través de E/S de bypass de núcleo y analítica paralela vectorizada. Funciona como un servicio de D-Bus que gestiona el ciclo de vida de los datos ingeridos en descriptores de archivos de memoria anónimos (`memfd`).

## Características Clave
- **Motor de Ingestión Dirac**: Una estrategia de ingestión de 3 niveles que utiliza `io_uring` (Nivel 1), `copy_file_range` (Nivel 2) y `mmap` (Nivel 3).
- **Axiom Crunch**: Cálculo de momentos estadísticos vectorizados (media, varianza, asimetría, curtosis) impulsado por Polars y Rayon.
- **Relational Strike**: Cálculo optimizado de la correlación de Pearson y Spearman.
- **Analítica SQL**: Ejecución directa de consultas SQL sobre datos residentes en memoria utilizando `polars-sql`.
- **Plano de Datos de Copia Cero**: Expone los resultados analíticos a través de la memoria compartida `iceoryx2` y D-Bus.

## Arquitectura
La refinería está estructurada en motores especializados:
- `DiracEngine`: Gestiona la E/S acelerada por hardware.
- `AnalyzeEngine`: Realiza cálculos estadísticos y ejecución de SQL.
- `DbusService`: Implementa la interfaz `org.nairobi.NairobiAxumRefinery1`.

## Instalación

### Requisitos Previos
- **Núcleo**: Linux 5.10+ (compatible con WSL2).
- **Dependencias**: `libdbus-1-dev`, `pkg-config`.
- **Huge Pages**: El motor funciona mejor con Huge Pages de 1GB habilitadas.
    ```bash
    echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
    ```

### Construir
```bash
cargo build --release -p nairobi-axum-refinery
```

## Desarrollo

### Configuración a Nivel de Núcleo
Los colaboradores deben tener en cuenta que `DiracEngine` intenta utilizar `IORING_SETUP_SQPOLL`. Para que esto funcione sin root, es posible que deba ajustar `/proc/sys/kernel/unprivileged_userns_clone` o ejecutar con `CAP_SYS_ADMIN`.

### Tutorial: Agregar una Nueva Métrica Estadística
1.  **Definir la Métrica**: En `src/analyze.rs`, actualice la estructura `StatisticalProfile` y su método `compute`.
2.  **Actualizar el Protocolo**: Agregue el nuevo campo a la estructura `DistilledAnalytics` en `crates/nairobi-protocol/src/types.rs`.
3.  **Exportar a través de D-Bus**: Asegúrese de que la interfaz de D-Bus en `src/dbus_service.rs` serialice correctamente el perfil actualizado.

## Pruebas
La refinería utiliza `tokio::test` para pruebas de integración asíncronas.
```bash
cargo test -p nairobi-axum-refinery
```

#### Mocking para Pruebas Aisladas
Puede probar el `AnalyzeEngine` de forma aislada creando un `memfd` manualmente y pasándolo al motor, omitiendo la capa de D-Bus:
```rust
let opts = memfd::MemfdOptions::default();
let mfd = opts.create("test.csv")?;
// Escribir datos de prueba...
let engine = AnalyzeEngine::new()?;
let results = engine.analyze(mfd.into_fd(), "target_column")?;
```

## Solución de Problemas
- **Fallo en la inicialización de `io_uring`**: Compruebe si su núcleo es compatible con `io_uring` (`zgrep CONFIG_IO_URING /proc/config.gz`).
- **Fallo en la asignación de Huge Pages**: Asegúrese de que el host tenga suficiente memoria contigua disponible. Verifique `grep Huge /proc/meminfo`.

## Soporte
Si encuentras útil Nairobi OS, considera apoyar el proyecto:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Licencia
Este proyecto está licenciado bajo la **Apache License 2.0**.

---
© 2026 Kevin Chege. Todos los derechos reservados.
