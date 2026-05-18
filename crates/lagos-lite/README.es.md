[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md)

# Lagos Vision (lagos-lite)

## Resumen
Lagos Vision es el motor de renderizado de alto rendimiento para Nairobi OS. Está diseñado para visualizar millones de puntos de datos con una latencia inferior a un milisegundo mediante el mapeo de memoria de los datos analíticos directamente en el canal de la GPU. Lagos funciona como un demonio sin cabezal que transmite tramas codificadas en JPEG a los widgets de los cuadernos Jupyter a través de WebSockets.

## Características Clave
- **Renderizado de Copia Cero**: Los datos se mapean en memoria desde los manejadores de `memfd` directamente a los búferes de `wgpu`.
- **Aceleración por Hardware**: Utiliza `egui` y `wgpu` (Vulkan, Metal, DX12 o OpenGL) para un trazado de alto rendimiento.
- **Downsampling LTTB**: Implementa el algoritmo Largest-Triangle-Three-Buckets en la GPU para mantener la precisión visual mientras se renderizan conjuntos de datos masivos.
- **Arquitectura Basada en Eventos**: Consume cero CPU cuando está inactivo; solo renderiza ante actualizaciones de datos o interacción del usuario.

## Arquitectura
Lagos Vision consta de:
- **Lagos Lite**: La biblioteca principal que proporciona el canal de renderizado.
- **Lagos Vision Daemon**: El proceso binario que gestiona la superficie `wgpu` y el servidor WebSocket.
- **Lagos Widget**: Un componente de Python `anywidget` que muestra la transmisión.

## Instalación

### Requisitos Previos
- **GPU**: Una GPU compatible con Vulkan (o OSMesa para el respaldo de software).
- **Bibliotecas del Sistema**: `libosmesa6-dev`, `mesa-utils`, `xvfb`.

### Construir
```bash
cargo build --release -p lagos-lite --bin lagos-vision-daemon
```

## Uso

### En Nairobi OS
Lagos se usa típicamente a través del método `SovereignFrame.plot()` en Python.

### Depuración Manual
Puede iniciar el demonio manualmente para probar el canal de renderizado:
```bash
./target/release/lagos-vision-daemon --fd <FD_INT> --width 1000 --height 400
```

## Desarrollo

### Implementación de una Capa de Visualización Personalizada
1.  **Modificar el Canal**: En `src/pipeline.rs`, defina sus sombreadores de vértices y fragmentos (WGSL).
2.  **Actualizar el Diseño del Búfer**: Mapee los datos entrantes de `memfd` a los grupos de enlace de su nuevo sombreador.
3.  **Integración de la IU**: Agregue elementos de control (deslizadores, botones) a la interfaz `egui` en `src/device.rs`.

### Entornos sin Cabezal
En entornos como Google Colab, Lagos usa `xvfb-run` o OSMesa para manejar la falta de una pantalla física:
```bash
xvfb-run -s "-screen 0 1024x768x24" ./target/release/lagos-vision-daemon ...
```

## Pruebas
Lagos incluye pruebas de integración visual que capturan tramas y las comparan con imágenes de referencia.
```bash
cargo test -p lagos-lite
```

## Solución de Problemas
- **Fallo en la Conexión WebSocket**: En entornos de nube (Colab/SageMaker), asegúrese de que el puerto proxy esté correctamente mapeado. Nairobi Python maneja esto automáticamente si se detecta `google.colab`.
- **Adaptador WGPU no Encontrado**: Asegúrese de que los controladores de la GPU estén instalados. Si usa un entorno solo de CPU, Lagos intentará recurrir a un adaptador de software.

## Licencia
Este proyecto está licenciado bajo la **Apache License 2.0**.

---
© 2026 Kevin Chege. Todos los derechos reservados.
