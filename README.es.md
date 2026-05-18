[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi OS

## Resumen
Nairobi OS es una infraestructura de ciencia de datos distribuida de alto rendimiento diseñada para una eficiencia de recursos extrema. Permite el procesamiento de conjuntos de datos masivos en entornos restringidos (Edge, IoT, Serverless) mediante el uso de un demonio de refinería especializado basado en Rust. Al utilizar características a nivel de núcleo como `io_uring`, `memfd` y Huge Pages, Nairobi OS logra una sobrecarga de IPC inferior a un milisegundo y canales de datos de copia cero.

## Características Clave
- **Ingestión de Copia Cero**: Carga de datos acelerada por hardware utilizando `io_uring` y Huge Pages de 1GB.
- **Visualización Acelerada por Hardware**: Trazado interactivo de Jupyter a través del motor Lagos Vision (`wgpu` y `egui`).
- **Canal de Analítica Fusionada**: Ingerir, procesar y correlacionar datos en un solo viaje de ida y vuelta de D-Bus.
- **Rendimiento de Bypass de Núcleo**: Analítica vectorizada que aprovecha Polars y Rayon para la máxima saturación de hardware.
- **Interfaz Sovereign**: Una API fluida de Python que oculta la complejidad del IPC de bajo nivel y la gestión de memoria.

## Arquitectura
Nairobi OS se basa en una tríada de componentes especializados conectados a través de D-Bus y memoria compartida:

1.  **Nairobi Axum Refinery**: El núcleo de Rust de alto rendimiento. Gestiona la ingestión de datos brutos y la analítica paralelizada.
2.  **Nairobi Hub**: El orquestador de IPC. Coordina los descriptores de archivos y las señales entre la refinería y los clientes.
3.  **Lagos Vision**: La corteza visual. Un motor de renderizado sin cabezal que mapea los manejadores de `memfd` directamente en el canal de la GPU.
4.  **Nairobi Python**: El puente de alto nivel. Proporciona una interfaz Pythonic para el ecosistema de Rust.

```text
[ Fuente de Datos ] -> (io_uring/Huge Pages) -> [ Axum Refinery ]
                                                     |
                                          (D-Bus / memfd / iceoryx2)
                                                     |
                                          [ Nairobi Hub ]
                                             /        \
                             [ Nairobi Python ]    [ Lagos Vision ]
                                     |                    |
                            [ Jupyter Notebook ] <-> [ Salida Visual ]
```

## Instalación

### Requisitos Previos
- **Sistema Operativo**: Linux o WSL2 (se requiere núcleo 5.10+ para `io_uring` y `memfd`).
- **Rust**: 1.70+
- **Python**: 3.10+
- **Bibliotecas del Sistema**:
    ```bash
    sudo apt-get update && sudo apt-get install -y \
        build-essential \
        pkg-config \
        libdbus-1-dev \
        python3-dev \
        dbus-x11 \
        libosmesa6-dev \
        mesa-utils
    ```

### Construir desde el Código Fuente
1. **Clonar el Repositorio**:
    ```bash
    git clone https://github.com/KevinKenya/nairobi-connector-open-source
    cd nairobi-connector-open-source
    ```

2. **Configurar el Entorno Virtual**:
    ```bash
    python3 -m venv .venv
    source .venv/bin/activate
    pip install maturin pyo3-build-config zbus anywidget traitlets
    ```

3. **Construir todo el Stack**:
    ```bash
    ./build_wheel.sh
    ```

4. **Instalar el Wheel**:
    ```bash
    pip install target/wheels/nairobi_os-0.3.1-py3-none-any.whl
    ```

## Configuración del Sistema (Guía del Colaborador)

### Huge Pages
El motor de la refinería prioriza las Huge Pages de 1GB para los búferes de copia cero. Para habilitarlas en su host:
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*Nota: Si las páginas de 1GB no están disponibles, el motor recurrirá automáticamente a Transparent Huge Pages (THP).*

### io_uring y SQPOLL
El `DiracEngine` utiliza `io_uring` con `SQPOLL` para el máximo rendimiento de E/S. `SQPOLL` generalmente requiere privilegios elevados (`CAP_SYS_ADMIN`) o un núcleo configurado con `IORING_SETUP_SQPOLL`. Si el motor no puede inicializar `SQPOLL`, recurrirá al modo `io_uring` estándar.

## Uso
```python
import nairobi_os

# Encender la refinería
nairobi_os.connect()

# Ingerir datos en un SovereignFrame
df = nairobi_os.read_csv("dataset.csv")

# Realizar analítica vectorizada
print(f"Media: {df.column_name.mean()}")

# Generar visualización interactiva
df.plot()
```

## Pruebas
Nairobi OS incluye un conjunto de pruebas exhaustivo que cubre unidades de Rust, integración de IPC y enlaces de Python.

### Ejecutar Todas las Pruebas
```bash
# Ejecutar pruebas de Rust
cargo test --workspace

# Ejecutar pruebas de integración de Python
python3 test_nairobi.py
```

### Benchmarking
Los benchmarks de rendimiento detallados se pueden ejecutar desde el directorio `nairobi-benchmarks`:
```bash
cd nairobi-benchmarks
pip install -r requirements.txt
python orchestration/benchmark_runner.py --workload workloads/workload_nba_pipeline.yaml
```

## Solución de Problemas
- **Conexión D-Bus Rechazada**: Asegúrese de que `dbus-daemon` esté ejecutándose. En entornos sin cabezal, use `dbus-launch`.
- **Problemas de Renderizado de Lagos**: Lagos requiere un controlador de GPU válido o OSMesa para el respaldo de software. Verifique con `glxinfo`.
- **Fallo en la Asignación de Huge Pages**: Verifique `/proc/meminfo` para asegurarse de que el núcleo haya reservado suficientes huge pages.

## Licencia
Este proyecto está licenciado bajo la **PolyForm Noncommercial License 1.0.0**. Es gratuito para uso personal, educativo y de investigación.

---
© 2026 Kevin Chege. Todos los derechos reservados.
