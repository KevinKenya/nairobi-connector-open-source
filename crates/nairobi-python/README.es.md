[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Nairobi Python

## Resumen
Nairobi Python proporciona el puente de alto nivel a la infraestructura de Nairobi OS. Permite a los científicos de datos aprovechar el poder de la analítica acelerada por hardware basada en Rust a través de una interfaz familiar y Pythonic. El paquete se encarga de la gestión del demonio, la coordinación de IPC y el mapeo de memoria, lo que permite a los usuarios centrarse en el análisis de datos.

## Características Clave
- **SovereignFrame**: Una interfaz fluida, similar a Pandas, para gestionar manejadores de datos remotos.
- **Ignición Diferida (Lazy Ignition)**: Inicia y configura automáticamente el demonio de la refinería tras el primer acceso a los datos.
- **Integración con Jupyter**: Soporte de primera clase para visualizaciones interactivas utilizando el widget Lagos Vision.
- **Puente de Copia Cero (Zero-Copy Bridge)**: Consume directamente manejadores de `memfd` de la refinería Rust con una sobrecarga inferior a un milisegundo.

## Instalación

### Desde PyPI
```bash
pip install nairobi-os
```

### Desde el Código Fuente
```bash
cd crates/nairobi-python
pip install -e .
```
*Nota: La construcción desde el código fuente requiere tener instalados el toolchain de Rust y `maturin`.*

## Uso

### Inicio Rápido
```python
import nairobi_os

# Conectarse a la refinería (gestiona automáticamente D-Bus y el inicio del demonio)
nairobi_os.connect()

# Ingerir un archivo CSV
df = nairobi_os.read_csv("data.csv")

# API fluida para estadísticas
mean_val = df.column_name.mean()
p99_val = df.column_name.p99()

# Ejecutar consultas SQL directamente en el motor
tall_players = df.query("SELECT * FROM dataset WHERE height > 80")

# Graficar usando Lagos Vision
tall_players.plot()
```

## Referencia de la API

### `nairobi_os.connect()`
Inicializa el entorno, inicia la sesión de D-Bus si es necesario y enciende el demonio de la refinería.

### `nairobi_os.read_csv(path, delimiter=",", encoding="utf-8")`
Ingiere un archivo CSV utilizando el canal de copia cero de la refinería. Devuelve un `SovereignFrame`.

### Métodos de `SovereignFrame`
- `df.column.mean()`: Calcula la media aritmética.
- `df.column.std_dev()`: Calcula la desviación estándar.
- `df.column.p95()`, `df.column.p99()`: Calcula percentiles.
- `df.column.skewness()`, `df.column.kurtosis()`: Calcula momentos estadísticos.
- `df.query(sql_string)`: Ejecuta Polars-SQL en el conjunto de datos.
- `df.correlate("col1,col2")`: Calcula la correlación de Pearson y Spearman.
- `df.plot(width, height)`: Muestra una visualización interactiva de `anywidget`.

## Desarrollo

### Agregar Nuevos Enlaces de Python
Nairobi Python utiliza PyO3 para interactuar con Rust. Las nuevas funciones principales deben agregarse a `crates/nairobi-python/src/lib.rs` y exponerse a través del módulo `nairobi_os._core.data`.

### Pruebas
Las pruebas de integración para el paquete Python se pueden ejecutar usando `pytest` (si está configurado) o el script de prueba proporcionado:
```bash
python3 test_nairobi.py
```

Para probar de forma aislada sin la refinería completa, puede simular el módulo `_core.data` o usar el `SovereignFrame` con manejadores preexistentes.

## Solución de Problemas
- **La refinería no se pudo registrar en D-Bus**: Esto suele suceder en entornos sin cabezal. Asegúrese de que `dbus-launch` esté disponible o llame a `nairobi_os.connect()`, que intenta corregir el entorno.
- **Manejador no encontrado**: Los manejadores de datos están vinculados a la sesión. Si la refinería se reinicia, los manejadores previos de `SovereignFrame` dejarán de ser válidos.

## Licencia
Este proyecto está licenciado bajo la **PolyForm Noncommercial License 1.0.0**.

---
© 2026 Kevin Chege. Todos los derechos reservados.
