[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Protocol

## Обзор
Crate **Nairobi Protocol** определяет общие интерфейсы D-Bus, сигнатуры GVariant и структуры данных, используемые во всей экосистеме Nairobi OS. Он служит «единственным источником истины» для обеспечения типизации и безопасности типов во всем ядре Rust, оркестраторе Hub и привязках Python (bindings).

## Ключевые компоненты
- **Определения интерфейсов**: Константы для имен служб, путей к объектам и сигнатур методов.
- **Общие типы**: GVariant-совместимые структуры, такие как `DistilledAnalytics` и `CorrelationResult`.
- **Управление памятью**: Обертка `MemoryPipe` для операций `memfd` и определения арен `iceoryx2`.

## Интерфейс D-Bus
- **Имя службы**: `org.nairobi.NairobiAxumRefinery1`
- **Путь к объекту**: `/org/nairobi/NairobiAxumRefinery1`
- **Интерфейс**: `org.nairobi.NairobiAxumRefinery1`

## Использование
Добавьте этот crate в качестве зависимости в любой компонент, которому требуется обмен данными в экосистеме Nairobi OS.

## Разработка
Любые изменения в этом crate должны вноситься с крайней осторожностью, поскольку они требуют повторной компиляции всех зависимых пакетов и могут нарушить бинарную совместимость между refinery и Python-привязками.

## Тестирование
Интеграционные тесты гарантируют, что сигнатуры GVariant соответствуют ожидаемому протоколу D-Bus:
```bash
cargo test -p nairobi-protocol
```

## Поддержка
Если вы находите Nairobi OS полезной, подумайте о поддержке проекта:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Лицензия
Этот проект лицензирован на условиях лицензии **Apache License 2.0**.

---
© 2026 Kevin Chege. All Rights Reserved.
