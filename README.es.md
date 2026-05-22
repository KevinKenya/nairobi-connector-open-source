[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi OS

## Descripción General
Nairobi OS es una infraestructura de ciencia de datos e IA distribuida de alto rendimiento diseñada para una eficiencia extrema de recursos. Permite el procesamiento de conjuntos de datos masivos en entornos restringidos (Edge, IoT, Serverless) mediante el uso de un demonio de refinería especializado basado en Rust, y proporciona **Uso del Ordenador sin píxeles** a través de su puente de accesibilidad compatible con MCP.

Al utilizar características a nivel del núcleo como `io_uring`, `memfd` y Huge Pages, Nairobi OS logra una sobrecarga IPC de sub-milisegundos y canales de datos de copia cero (zero-copy).

## Demostración

https://github.com/user-attachments/assets/demo.webm

<video src="assets/demo.webm" controls width="100%">
  Tu navegador no soporta el elemento de video.
</video>

## Características Principales
- **Uso del Ordenador Sin Píxeles**: Interactúa directamente con el escritorio Linux a través de AT-SPI2 y el algoritmo de compresión TOON (Token-Oriented Object Notation), omitiendo el procesamiento visual u OCR para agentes de IA.
- **Ingesta de Copia Cero**: Carga de datos acelerada por hardware mediante `io_uring` y Huge Pages de 1GB.
- **Visualización Acelerada por Hardware**: Gráficos interactivos de Jupyter a través del motor Lagos Vision (`wgpu` y `egui`).
- **Canal de Analítica Fusionada**: Ingiere, procesa y correlaciona datos en un solo viaje de ida y vuelta D-Bus.
- **Rendimiento de Omisión del Núcleo**: Analítica vectorizada aprovechando Polars y Rayon.
- **Interfaz Soberana**: Una API de Python fluida que oculta la complejidad del IPC y la gestión de memoria.

## Arquitectura
Nairobi OS se basa en componentes especializados conectados a través de D-Bus y memoria compartida:

1.  **Nairobi Axum Refinery**: El núcleo Rust de alto rendimiento.
2.  **Nairobi Hub**: El orquestador IPC.
3.  **Lagos Vision**: El motor de renderizado "headless".
4.  **Nairobi Connector**: El puente semántico y servidor MCP.
5.  **Nairobi Python**: El puente de alto nivel. Proporciona una interfaz Python al ecosistema Rust.

## Instalación

### Desde PyPI
```bash
pip install nairobi-os
```

### Compilar desde la Fuente
```bash
git clone https://github.com/KevinKenya/nairobi-connector-open-source
cd nairobi-connector-open-source
python3 -m venv .venv
source .venv/bin/activate
pip install maturin pyo3-build-config zbus anywidget traitlets
./build_wheel.sh
```

## Uso

### Analítica de Datos
```python
import nairobi_os

# Iniciar la refinería
nairobi_os.connect()

# Ingerir datos
df = nairobi_os.read_csv("dataset.csv")

print(f"Media: {df.column_name.mean()}")
df.plot()
```

### Uso del Ordenador (Servidor MCP)
Los agentes de IA que usan Nairobi Connector deben seguir este bucle:
1. Apuntar a una ventana usando `nairobi_find_window`.
2. Observar el estado actual vía `nairobi_get_ui_map`.
3. Leer el TOON `[ID: N]` del elemento deseado.
4. Ejecutar una acción vía `nairobi_interact` o `nairobi_type_text`.

## Soporte
Si encuentras útil Nairobi OS, considera apoyar el proyecto:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Licencia
Este proyecto está licenciado bajo la **Licencia Apache 2.0**.
*(Nota: Partes del formato TOON y la implementación se atribuyen a los Autores de TOON).*

---
© 2026 Kevin Chege. Todos los derechos reservados.
