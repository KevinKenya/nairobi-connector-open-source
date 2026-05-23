[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Axum Refinery

## Обзор
**Axum Refinery** — это высокопроизводительное ядро Nairobi OS. Написанный на Rust, он разработан для полной загрузки современного оборудования посредством ввода-вывода в обход ядра (kernel-bypass I/O) и векторизованной параллельной аналитики. Он функционирует как служба D-Bus, которая управляет жизненным циклом данных, загружаемых в дескрипторы анонимных файлов памяти (`memfd`).

## Ключевые особенности
- **Движок Dirac Ingestion**: Трехуровневая стратегия загрузки данных с использованием `io_uring` (уровень 1), `copy_file_range` (уровень 2) и `mmap` (уровень 3).
- **Axiom Crunch**: Векторизованный расчет статистических моментов (среднее значение, дисперсия, асимметрия, эксцесс) на базе Polars и Rayon.
- **Relational Strike**: Оптимизированный расчет коэффициентов корреляции Пирсона и Спирмена.
- **SQL-аналитика**: Прямое выполнение SQL-запросов к размещенным в памяти данным с использованием `polars-sql`.
- **Zero-Copy Data Plane**: Передача результатов аналитики через общую память `iceoryx2` и D-Bus без копирования.

## Архитектура
Refinery состоит из специализированных движков:
- `DiracEngine`: Управляет аппаратно ускоренным вводом-выводом.
- `AnalyzeEngine`: Выполняет статистические расчеты и SQL-запросы.
- `DbusService`: Реализует D-Bus интерфейс `org.nairobi.NairobiAxumRefinery1`.

## Установка и настройка

### Требования
- **Ядро**: Linux 5.10+ (поддерживается в WSL2).
- **Системные зависимости**: `libdbus-1-dev`, `pkg-config`.
- **Огромные страницы памяти (Huge Pages)**: Движок демонстрирует максимальную производительность при включении страниц размером 1 ГБ.
    ```bash
    echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
    ```

### Сборка
```bash
cargo build --release -p nairobi-axum-refinery
```

## Разработка

### Конфигурация на уровне ядра
Разработчикам следует учитывать, что `DiracEngine` пытается использовать флаг `IORING_SETUP_SQPOLL`. Для того чтобы это работало без прав root, вам может потребоваться настроить параметр `/proc/sys/kernel/unprivileged_userns_clone` или запустить процесс с правами `CAP_SYS_ADMIN`.

### Руководство: Добавление новой статистической метрики
1.  **Определение метрики**: В файле `src/analyze.rs` обновите структуру `StatisticalProfile` и ее метод `compute`.
2.  **Обновление протокола**: Добавьте новое поле в структуру `DistilledAnalytics` в файле `crates/nairobi-protocol/src/types.rs`.
3.  **Экспорт через D-Bus**: Убедитесь, что D-Bus интерфейс в `src/dbus_service.rs` корректно сериализует обновленный профиль.

### Тестирование
Refinery использует `tokio::test` для асинхронного интеграционного тестирования.
```bash
cargo test -p nairobi-axum-refinery
```

#### Изолированное тестирование с использованием заглушек (Mocking)
Вы можете протестировать `AnalyzeEngine` в изоляции, вручную создав `memfd` и передав его движку в обход D-Bus:
```rust
let opts = memfd::MemfdOptions::default();
let mfd = opts.create("test.csv")?;
// Запись тестовых данных...
let engine = AnalyzeEngine::new()?;
let results = engine.analyze(mfd.into_fd(), "target_column")?;
```

## Устранение неполадок
- **Сбой инициализации `io_uring`**: Проверьте, поддерживает ли ваше ядро `io_uring` (`zgrep CONFIG_IO_URING /proc/config.gz`).
- **Сбой выделения Huge Pages**: Убедитесь, что на хосте достаточно свободной непрерывной памяти. Проверьте через `grep Huge /proc/meminfo`.

## Поддержка
Если вы находите Nairobi OS полезной, подумайте о поддержке проекта:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Лицензия
Этот проект лицензирован на условиях лицензии **Apache License 2.0**.

---
© 2026 Kevin Chege. All Rights Reserved.
