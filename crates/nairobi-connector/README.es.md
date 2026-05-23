[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Connector

## Descripción General
**Nairobi Connector** es el puente semántico AT-SPI2 y servidor del Protocolo de Contexto de Modelos (MCP) para Nairobi OS. Permite el **"Uso del Ordenador sin píxeles"** (Computer Use without pixels) exponiendo el árbol de accesibilidad del escritorio de Linux a LLMs y agentes de IA en un formato hiperdenso y optimizado para tokens conocido como TOON (Token-Oriented Object Notation). Al operar puramente en nodos UI semánticos en lugar de píxeles, logra una ejecución de acciones casi instantánea y reduce drásticamente el consumo de tokens.

## Características Principales
- **Uso del Ordenador Sin Píxeles**: Interactúa directamente con el escritorio Linux a través de AT-SPI2, eliminando la necesidad de capturas de pantalla, OCR o procesamiento visual.
- **Algoritmo de Compresión TOON**: Traduce los árboles de accesibilidad de D-Bus a una representación Markdown altamente comprimida. Filtra los nodos "ruido" no interactivos y asigna IDs secuenciales a elementos accionables.
- **Integración de Servidor MCP**: Implementa un servidor basado en `rmcp` que expone herramientas semánticas de forma nativa a agentes LLM compatibles.
- **Ciclo de Vida de Sesión Seguro**: Cuenta con un monitor de latidos (heartbeat) para evitar la parálisis del SO, liberando automáticamente `RegistryLock` si la tubería `stdio` se bloquea.

## Arquitectura
El conector actúa como un puente bidireccional entre los LLMs (vía MCP) y el escritorio Linux (vía AT-SPI2/D-Bus).

### Herramientas MCP Proporcionadas
- `nairobi_find_window`: Encuentra y selecciona una ventana por subcadena del título.
- `nairobi_get_ui_map`: Devuelve el árbol de accesibilidad actual como un mapa comprimido TOON. Genera una lista densa de elementos interactivos con etiquetas `[ID: N]` secuenciales para apuntar.
- `nairobi_interact`: Ejecuta acciones semánticas (`click`, `activate`, `focus`) en un elemento de la interfaz utilizando su ID de nodo TOON.
- `nairobi_type_text`: Inyecta texto de forma atómica en un campo editable (Entry, TextArea).

## Uso
Los agentes que utilizan Nairobi Connector deben seguir este bucle fundamental:
1. Buscar una ventana usando `nairobi_find_window`.
2. Observar el estado actual a través de `nairobi_get_ui_map`.
3. Leer el `[ID: N]` del elemento interactivo deseado.
4. Ejecutar una acción sobre ese elemento vía `nairobi_interact` o `nairobi_type_text`.
5. Repetir desde el paso 2 para obtener IDs nuevos antes de volver a interactuar.

## Soporte
Si encuentras útil Nairobi OS, considera apoyar el proyecto:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Licencia
Este proyecto está licenciado bajo la **Licencia Apache 2.0**.
*(Nota: Partes del formato TOON y la implementación del puente se atribuyen a los autores de TOON).*
