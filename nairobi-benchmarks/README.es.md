[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Suite de Benchmarks de Nairobi (Nairobi Benchmark Suite)

## Resumen
La Suite de Benchmarks de Nairobi es un marco de evaluación de rendimiento riguroso diseñado para comparar Nairobi OS con bibliotecas de procesamiento de datos estándar de la industria (p. ej., Pandas). Se centra en la latencia de extremo a extremo, la eficiencia de la memoria y el impacto de los "Ataques Analíticos Fusionados" en las cargas de trabajo del mundo real.

## Métricas Clave
- **Latencia de Ingestión**: Tiempo para cargar datos desde el disco en estructuras residentes en memoria.
- **Densidad de Cómputo**: Tamaño máximo del conjunto residente (RSS) durante cargas analíticas pesadas.
- **Rendimiento del Canal**: Tiempo total para las operaciones fusionadas de ingestión-procesamiento-correlación.

## Instalación

### Requisitos Previos
- Python 3.10+
- Nairobi OS (instalado y configurado)

### Configuración
```bash
cd nairobi-benchmarks
pip install -r requirements.txt
```

## Ejecución de Benchmarks

### 1. Preparar Conjuntos de Datos
Generar conjuntos de datos sintéticos para probar la escala:
```bash
# Generar conjunto de datos de 10 millones de filas
python datasets/generators/generate_synthetic.py --type tall --output datasets/synthetic/tall_10m.csv
```

### 2. Ejecutar Cargas de Trabajo
Ejecutar una carga de trabajo de benchmark específica:
```bash
python orchestration/benchmark_runner.py --workload workloads/workload_statistical_distillation.yaml --iterations 10
```

### 3. Analizar Resultados
Los resultados de los benchmarks se almacenan en formato JSON y se pueden visualizar utilizando las herramientas de trazado incluidas:
```bash
python visualization/plot_scaling.py
```

## Metodología
La suite sigue una metodología de benchmarking "Hardware-First", asegurando que:
- Los inicios en frío y en caliente se miden por separado.
- Las cachés del núcleo se limpian (donde sea posible) entre ejecuciones.
- Todos los cálculos se verifican para la identidad matemática (±1e-5) utilizando `result_validator.py`.

## Licencia
Esta suite es parte del proyecto Nairobi OS y está licenciada bajo la **Apache License 2.0**.

---
© 2026 Kevin Chege. Todos los derechos reservados.
