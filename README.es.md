[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md) | [Finnish](README.fi.md) | [Cantonese](README.yue.md) | [Français](README.fr.md) | [Nederlands](README.nl.md)

# Nairobi OS: Infraestructura de IA y Ciencia de Datos de Alto Rendimiento y Copia Cero

[![PyPI Version](https://img.shields.io/pypi/v/nairobi-os.svg)](https://pypi.org/project/nairobi-os/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
![System](https://img.shields.io/badge/Kernel-Linux_6.17_Native-orange.svg)
![Arch](https://img.shields.io/badge/Architecture-x86__64_/_ARM64-red.svg)

---

## El Origen: Del Crisol al Metal


Mi camino en la programación está arraigado en la arquitectura de sistemas de bajo nivel y la optimización extrema. En 2015, expuse mi visión para construir capacidades técnicas altamente descentralizadas en el continente africano en [este tratado sobre el Silicon Valley de Kenia](https://www.linkedin.com/pulse/building-kenyas-silicon-valley-making-work-kevin-chege/). Cuando comenzó la fiebre del oro de los LLM en 2023, fui de los primeros. Construí y desplegué wrappers de LLM, pero rápidamente reconocí sus limitaciones, tal como se documenta en esta temprana [demostración de wrapper de LLM de 2023](https://www.linkedin.com/feed/update/urn:li:activity:7102930955807449088/).

Me di cuenta de que construir wrappers de alto nivel sobre APIs inestables era un callejón sin salida arquitectónico. La verdadera guerra se libra en la intersección de las limitaciones de hardware local y la asignación de recursos.

A lo largo de 2025, viví en una Lenovo X13 ThinkPad con un perfil de hardware altamente restringido:

```
Procesador: AMD Ryzen 5 PRO 4650U (6 núcleos, 12 hilos)
Gráficos: AMD Radeon RX Vega 6 iGPU
Memoria: 32 GB RAM (con una utilización extremadamente alta del sistema)
Almacenamiento: 256 GB NVMe (99% lleno)
```

En esta máquina exacta, pasé el año 2025 construyendo **Tumz** ([Sarafakai](http://www.sarafakai.com)), una IA de soporte de decisiones clínicas en tiempo real, sin latencia y aislada del exterior (air-gapped). Ejecutaba transcripción de audio en vivo y en tiempo real junto con inferencia clínica de forma simultánea en la GPU integrada (iGPU), manteniendo todo el Sistema de Lenguaje Médico Unificado (UMLS) residente en RAM. Actualmente nos estamos asociando con un hospital keniano para pilotar Tumz en un ensayo clínico de un año, porque la salud humana requiere una validación rigurosa y empírica, no las suposiciones de los desarrolladores.

Durante el desarrollo de Tumz, me encontré con las ineficiencias masivas y sistémicas del stack de ciencia de datos moderno:
1. **El Impuesto de Python**: Copia de memoria de extremo a extremo, cuellos de botella del GIL y un enorme consumo de tiempo de ejecución.
2. **El Impuesto del Navegador**: Complicaciones de Manifest V3, latencia de renderizado y fallos de comunicación de alta frecuencia en conversaciones de agentes de larga duración.
3. **El Cuello de Botella del Núcleo del SO**: Programación ineficiente de procesos, inanición de hilos de CPU y sobrecarga del servidor de pantalla (cambio de contexto Wayland vs. X11).

Así, a finales de 2025, me propuse construir un stack de infraestructura que sorteara estos límites por completo: un Sistema Operativo Agéntico (Agentic OS) diseñado para pipelines de datos de copia cero y ejecución nativa en hardware de IA. Este repositorio es el núcleo de código abierto de ese motor.

---

## El Registro de Batalla: 9,180 Contribuciones en el Último Año

Algunos críticos en la comunidad de desarrollo moderna miran los proyectos nuevos y altamente avanzados y los rechazan como "boilerplate generado por IA". A esos escépticos, les ofrezco la prueba cruda y física del registro de commits.

Mi otro perfil de GitHub (https://github.com/ChegeKenya) se establece como un registro empírico de intensa ingeniería de sistemas diaria. En 2025 solo, registré 7,888 contribuciones. En los primeros cinco meses de 2026, agregué 1,420 contribuciones. Eso es 9,180 contribuciones en los últimos 365 días — una secuencia casi ininterrumpida de commits verdes que abarcan runtimes Rust de baja latencia, pipelines de IA clínica y sistemas de memoria compartida de copia cero. Este código está escrito en las trincheras, compilado en bare metal y auditado byte por byte.

```
2025: [██████████████████████████████████████████████████] 7,888 Commits
  2026: [██████████] 1,420 Commits
  Total (Último Año): 9,180 Commits de Código de Sistemas Puros
```

### Restricciones de Desarrollo Nativas de Hardware

Mi trayectoria en programación está arraigada en la arquitectura de sistemas de bajo nivel y la optimización extrema, mecánica. En 2015, compartí mi trayectoria en programación, y expuse mi visión para construir capacidades técnicas altamente descentralizadas en el continente africano en [este tratado sobre el Silicon Valley de Kenia](https://www.linkedin.com/pulse/building-kenyas-silicon-valley-making-work-kevin-chege/). Cuando la transición LLM comenzó en 2023, reconocí temprano las limitaciones estructurales de los wrappers de alto nivel, como se documenta en mi temprana [demostración de wrapper LLM de 2023](https://www.linkedin.com/feed/update/urn:li:activity:7102930955807449088/).

Me di cuenta de que construir wrappers de alto nivel sobre APIs web inestables era un callejón sin salida arquitectónico. La verdadera guerra se libra en la intersección de las limitaciones de hardware local y la asignación directa de recursos.

A lo largo de 2025 y 2026, viví y desarrollé en un Lenovo X13 ThinkPad altamente restringido:

```
Procesador: AMD Ryzen 5 PRO 4650U (6 núcleos, 12 hilos)
Gráficos: AMD Radeon RX Vega 6 iGPU (Arquitectura de Memoria Compartida)
Memoria: 29 GiB RAM (con una utilización del sistema alta)
Almacenamiento: 256 GB NVMe (99% lleno, altamente restringido)
Kernel del host: Linux 6.17.0-29-generic
```

En esta máquina exacta, pasé 2025 construyendo **Tumz** ([Sarafakai](http://www.sarafakai.com)), una IA de soporte de decisiones clínicas air-gapped, de latencia cero. Ejecutaba transcripción de audio en vivo y en tiempo real junto con inferencia clínica simultáneamente en la GPU integrada (iGPU), manteniendo todo el diccionario del Unified Medical Language System (UMLS) residente en RAM compartida. Sarafakai ahora se está asociando con un hospital keniano para pilotar Tumz en un ensayo clínico de un año — porque la salud humana requiere validación rigurosa y empírica, no suposiciones de desarrolladores.

---

## Tracción Global y Telemetría

Lanzado el 6 de mayo de 2026, Nairobi OS ha ganado tracción rápidamente entre programadores de sistemas, investigadores cuantitativos y arquitectos de edge computing de todo el mundo. Estas estadísticas de descarga se obtienen del [Panel de Control en Vivo de ClickPy Nairobi OS](https://clickpy.clickhouse.com/dashboard/nairobi-os), donde puede buscar y explorar las métricas por sí mismo.

### Distribución Global Acumulada (6 de mayo de 2026 – 23 de mayo de 2026)

| Métrica | Medición | Contexto |
| :--- | :--- | :--- |
| **Rango Global** | **#75,293** | De 797,894 paquetes activos en PyPI |
| **Percentil** | **9.43%** | Clasificación de primer nivel para extensiones de Python a nivel de sistema |
| **Descargas Totales** | **1,525** | Descargas de desarrolladores orgánicas y de alta intención |

### Volumen de Descargas por Versión

```
  0.2.0 [████████████████████████████████████████] 342
  0.2.1 [██████████████████████████] 224
  0.3.0 [████████████████████████] 212
  0.3.1 [████████████████████] 176
  0.1.0 [███████████████████] 169
  0.4.1 [██████████████] 120
```

### Top 10 Regiones Soberanas de Adopción

| Rango | Región | Código de País | Volumen de Descargas |
| :--- | :--- | :--- | :--- |
| 1 | Estados Unidos | US | 661 |
| 2 | Hong Kong | HK | 103 |
| 3 | China | CN | 84 |
| 4 | Alemania | DE | 74 |
| 5 | Japón | JP | 65 |
| 6 | Singapur | SG | 56 |
| 7 | Reino Unido | GB | 51 |
| 8 | Francia | FR | 51 |
| 9 | Rusia | RU | 42 |
| 10 | Corea del Sur | KR | 30 |

---

## Soporte y Soberanía

Si Nairobi OS está optimizando sus pipelines de datos, reduciendo sus facturas de la nube o impulsando sus arquitecturas agénticas locales, considere apoyar nuestra investigación de sistemas independiente. Cada contribución se destina directamente a optimizaciones de compiladores a nivel de hardware y pruebas de edge computing en Nairobi.

[![Apoyar el Desarrollo de Nairobi OS](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

---

## Características Clave

* **Uso del Ordenador sin Píxeles**: Evita los lentos y costosos pipelines de agentes basados en visión. Interactúa de forma nativa con el escritorio Linux a través de AT-SPI2 y el algoritmo de compresión TOON (Token-Oriented Object Notation), alimentando árboles jerárquicos crudos directamente a los LLM.
* **Ingesta de Copia Cero**: Carga de datos acelerada por hardware y con omisión del núcleo utilizando `io_uring` y Huge Pages de 1GB.
* **Visualización Acelerada por Hardware**: Gráficos interactivos de Jupyter de baja latencia utilizando el demonio de renderizado `lagos-lite`, construido sobre `wgpu` y `egui`.
* **Ejecución Analítica Vectorizada**: Saturación extrema de la CPU mediante la ejecución de consultas de Polars y pipelines de datos multihilo de Rayon.
* **Interfaz Soberana**: Una API de Python fluida (`SovereignFrame`) que encapsula descriptores de archivos mapeados en memoria e IPC.

---

## Arquitectura de Código Abierto vs. Empresarial

Nairobi OS está bifurcado estructuralmente. El repositorio de código abierto proporciona el procesamiento de datos de alto rendimiento fundamental y las primitivas de visualización en un solo nodo. El ecosistema comercial de código cerrado contiene las implementaciones avanzadas multi-agente, de alta disponibilidad y específicas de la industria.

```
                                  +---------------------------------------+
                                  |         Nairobi Python API            |
                                  +---------------------------------------+
                                                      |
                                     [ GVariant sobre D-Bus / memoria comp. ]
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
     |   Axum Refinery (Datos)      | <===[ Copia Cero IPC / iceoryx2 ]==>|     Lagos Vision (Visual)    |
     +------------------------------+                                    +------------------------------+
```

### Open Source Crate Workspace (`crates/`)
1. `nairobi-axum-refinery` — Rust daemon managing raw data ingestion, Rayon-parallelized statistics, and Polars-vectorized query execution.
2. `nairobi-hub` — Central IPC orchestrator; routes file descriptors and signals between clients and the refinery daemon.
3. `lagos-lite` — Local/headless rendering engine using egui/wgpu hardware acceleration with zero-copy mmap data access.
4. `nairobi-protocol` — Shared protocol layer: GVariant serialization schemes, error types, and shared-memory layouts.
5. `nairobi-python` — The Python extension module, compiled via PyO3 and packaged with Maturin (`nairobi-os`).
6. `nairobi-canvas` — Immediate-mode node-graph compiler with hardware-accelerated UI (wgpu/egui), including a native file picker and SQL query presets.
7. `nairobi-connector` — Model Context Protocol (MCP) server and AT-SPI2 semantic accessibility bridge exposing TOON representations for LLM agents.
### Ecosistema Corporativo Privado (`modules/`)

Nuestros componentes de nivel empresarial se encuentran en un repositorio privado (`Sovereign-Systems-Lab`) y tienen licencia para infraestructura industrial, financiera y estatal.

1. **`sovereign-ui`**: El motor empresarial AT-SPI2. Implementa la seguridad del Protocolo Aegis, vinculación de hardware y manipulación de escritorio de nivel de producción.
3. **`tactical-rtos-node`**: Programador de sistema operativo en tiempo real de ultra baja latencia para automatización industrial edge crítica para la seguridad.
4. **`industrial-guardian-rust` / `industrial-guardian-python`**: Capa autónoma de ingeniería de fiabilidad del sitio (SRE) con evitación predictiva de OOM, fugas de memoria y caídas del sistema.
5. **`fintech-bridge-rust`**: Analizador de transacciones de alta frecuencia en tiempo real y puente de mainframe heredado (análisis de terminales EBCDIC/SBA).
6. **`aviation-audio-rust`**: Procesamiento de flujo de audio sin bloqueos de sub-milisegundos, análisis de telemetría acústica y DSP de ondas crudas.
7. **`drawbridge_api`**: Puente levadizo gRPC multi-inquilino, seguro y autenticado, que aísla el núcleo local de las llamadas de agentes en la nube no confiables.

### Matriz de Comparación de Capacidades

| Capacidad / Característica | Código Abierto Core (`crates/`) | Suite Empresarial (`modules/`) |
| :--- | :---: | :---: |
| **Motor de Ingesta** | `mmap` / `copy_file_range` | `io_uring` + `SQPOLL` + 1GB Huge Pages |
| **Análisis Estadístico** | Estadísticas descriptivas básicas | Sesgo/curtosis multi-paso vectorizado, correlación |
| **Motor de Consultas** | SQL de Polars en proceso | Apache Arrow / DataFusion cluster distribuido |
| **Mecanismo IPC** | Memoria compartida POSIX / D-Bus | Memoria compartida `iceoryx2` copia cero |
| **Visualización** | Jupyter `anywidget` local | WebRTC GStreamer / overlays transparentes de Wayland Layer-Shell |
| **Seguridad y Cumplimiento** | Límites estándar de POSIX | Protocolo Aegis, Libro Mayor Forense Encadenado SHA-256 |
| **Autenticación** | Ninguna (Usuario local de confianza) | Vinculación de hardware (TPM 2.0 / CPU ID), PKI privada |
| **Objetivo de Plataforma** | Linux de un solo nodo | Nube distribuida / Nodo edge / Trading de alta frecuencia |

---

## Instalación y Configuración

### Requisitos
- **SO**: Linux (se recomienda Ubuntu 22.04+) o Windows Subsystem for Linux (WSL2).
- **GPU**: Controlador compatible con Vulkan, Metal o OpenGL.
- **Python**: 3.10 o más reciente.
- **Rust**: Cadena de herramientas estable (si se construye desde la fuente).

### Instalación Rápida (PyPI)
```bash
pip install nairobi-os
```

### Construir desde la Fuente
Para compilar todo el espacio de trabajo, incluidos los demonios nativos y la extensión de Python:

1. **Clonar el Repositorio**:
   ```bash
   git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
   cd nairobi-connector-open-source
   ```

2. **Configurar el Entorno Virtual**:
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   pip install maturin pyo3-build-config zbus anywidget traitlets pandas
   ```

3. **Ejecutar la Construcción del Espacio de Trabajo**:
   ```bash
   chmod +x build_wheel.sh
   ./build_wheel.sh --release
   ```
   Esto compila los demonios nativos, los copia al directorio del paquete y construye un wheel bajo `crates/nairobi-python/target/wheels/`.

---

## Guía de Uso

### 1. Análisis de Datos (El Pipeline En Memoria)

Nairobi OS proporciona la API `SovereignFrame`. Maneja el mapeo de memoria cruda bajo el capó, permitiendo una rápida manipulación de datos.

```python
import nairobi_os as nb

# Encender el demonio refinery de fondo
nb.connect()

# Ingerir el conjunto de datos usando el pipeline de memoria de copia cero
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# Realizar cálculos vectorizados a través de Rust refinery
profile = frame.crunch("value")
print(f"Media: {profile['mean']:.4f}")
print(f"Desviación Estándar: {profile['std_dev']:.4f}")

# Ejecutar consultas SQL arbitrarias directamente en el frame mapeado en memoria
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# Lanzar el widget de trazado interactivo acelerado por Lagos
subset.plot(column="value")
```

### 2. Uso del Ordenador sin Píxeles (MCP)

Para usar la interfaz semántica AT-SPI2, su agente de IA debe interactuar con las herramientas expuestas del servidor MCP en lugar de leer capturas de pantalla:

```
                    SECUENCIA DE USO DEL ORDENADOR
                     
  [ Agente LLM ]                                [ Nairobi OS ]
        |                                             |
        |===> nairobi_find_window("Text Editor") ====>| (Localiza el objetivo)
        |<=== Retorna ID de Ventana y Límites =======|
        |                                             |
        |===> nairobi_get_ui_map() ==================>| (Genera TOON)
        |<=== Retorna árbol Markdown comprimido ======|
        |     "[ID: 12] Button: 'Save'"               |
        |                                             |
        |===> nairobi_interact(12, "click") =========>| (Ejecuta la acción)
        |<=== Retorna estado de éxito ================|
```

---

## Ajuste del Sistema (Guía del Colaborador)

Para lograr los perfiles de rendimiento mostrados en nuestros benchmarks, su kernel host debe estar configurado para el mapeo de memoria a nivel de sistema.

### Huge Pages de 1GB
Nairobi OS utiliza Huge Pages de 1GB para evitar la sobrecarga de traducción de Translation Lookaside Buffer (TLB) de la CPU en conjuntos de datos masivos.

Para asignar una Huge Page en su host Linux:
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*Nota: Si el sistema no puede asignar una página de 1GB debido a la fragmentación, el motor vuelve automáticamente a Transparent Huge Pages (THP).*

### Configuración de D-Bus Broker
In entornos de alta frecuencia, asegúrese de que `dbus-broker` esté instalado en lugar del tradicional `dbus-daemon` para manejar una rápida propagación de señales a través del plano de control.

---

## Licencia

Este proyecto está licenciado bajo la **Licencia Apache 2.0**.  
*(Nota: Partes del formato TOON y de la implementación del puente se atribuyen a los autores de TOON).*

---
© 2026 Kevin Chege. Todos los derechos reservados.  
*Sovereign Systems Lab, Nairobi, Kenia.*
