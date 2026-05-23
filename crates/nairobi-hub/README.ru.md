[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Hub

## Обзор
**Nairobi Hub** — это центральный IPC-оркестратор (межпроцессное взаимодействие) для Nairobi OS. Он управляет координацией файловых дескрипторов, сигналов D-Bus и сегментов общей памяти между высокопроизводительным Rust-демоном refinery и его клиентами.

## Ключевые особенности
- **FD Proxying**: Безопасно передает файловые дескрипторы `memfd` через D-Bus с использованием сигнатур GVariant.
- **Управление службами**: Отслеживает и управляет жизненным циклом службы `org.nairobi.NairobiAxumRefinery1`.
- **Гибридный уровень данных (Data Plane)**: Динамически направляет данные через общую память `iceoryx2` (для максимальной производительности) или D-Bus (для широкой совместимости).
- **Семантическое декодирование**: Преобразует необработанные бинарные аналитические отчеты в удобные для чтения форматы и нативные структуры Python.

## Архитектура
Оркестратор Hub разделен на несколько внутренних модулей:
- `client.rs`: Прокси-клиент D-Bus.
- `shm_subscriber.rs`: Обрабатывает подписки на общую память `iceoryx2`.
- `decoder.rs`: Преобразует результаты GVariant в Markdown и JSON.

## Использование
Hub в основном используется в качестве внутренней библиотеки расширением `nairobi-python` для связи с refinery.

## Разработка
При модификации Hub убедитесь, что любые изменения в интерфейсе D-Bus также синхронизированы и отражены в crate `nairobi-protocol`.

## Тестирование
Интеграционные тесты для Hub проверяют полный цикл прохождения IPC-сигналов и данных:
```bash
cargo test -p nairobi-hub
```

## Поддержка
Если вы находите Nairobi OS полезной, подумайте о поддержке проекта:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Лицензия
Этот проект лицензирован на условиях лицензии **Apache License 2.0**.

---
© 2026 Kevin Chege. All Rights Reserved.
