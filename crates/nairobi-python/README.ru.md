[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi OS (nairobi-python)

## Обзор
**Nairobi OS** — это высокопроизводительная распределенная инфраструктура искусственного интеллекта и анализа данных, разработанная для экстремальной эффективности использования ресурсов. Используя специализированный демон refinery на базе Rust, она позволяет обрабатывать массивные наборы данных в ограниченных средах (Edge, IoT, Serverless) и обеспечивает **«использование компьютера без пикселей»** с помощью семантического моста доступности, совместимого с MCP.

Благодаря использованию таких возможностей ядра, как `io_uring`, `memfd` и Huge Pages, Nairobi OS обеспечивает субмиллисекундные накладные расходы IPC и конвейеры данных с нулевым копированием (zero-copy).

## Ключевые особенности
- **Использование компьютера без пикселей**: Прямое взаимодействие с рабочим столом Linux через AT-SPI2 и алгоритм сжатия TOON (Token-Oriented Object Notation), исключающее необходимость в скриншотах, OCR или визуальной обработке для ИИ-агентов.
- **Загрузка данных с нулевым копированием (Zero-Copy)**: Аппаратно ускоренная загрузка данных с использованием `io_uring` и огромных страниц памяти (Huge Pages) размером 1 ГБ.
- **Аппаратно ускоренная визуализация**: Интерактивное построение графиков Jupyter с малой задержкой с помощью движка Lagos Vision (`wgpu` и `egui`).
- **Слияние аналитических расчетов (Fused Analytics)**: Импорт, расчет статистических моментов и корреляции за один цикл D-Bus IPC.
- **Производительность в обход ядра (Kernel-Bypass)**: Векторизованная аналитика с использованием Polars и Rayon для максимальной загрузки аппаратных ресурсов.
- **Суверенный интерфейс**: Удобный API Python (`SovereignFrame`), скрывающий детали низкоуровневого IPC и управления памятью.

## Архитектура
Nairobi OS построена на триаде специализированных компонентов, соединенных через D-Bus и общую память:
1. **Nairobi Axum Refinery**: Высокопроизводительное ядро на Rust. Управляет импортом необработанных данных и параллельной аналитикой.
2. **Nairobi Hub**: IPC-оркестратор. Координирует файловые дескрипторы и сигналы между refinery и клиентами.
3. **Lagos Vision**: Визуальный интерфейс. Движок рендеринга, проецирующий файлы `memfd` напрямую в графический конвейер (GPU pipeline).
4. **Nairobi Connector**: Семантический мост. Сервер MCP, открывающий доступ к дереву доступности рабочего стола Linux для LLM.
5. **Nairobi Python**: Высокоуровневый интерфейс. Предоставляет удобный Pythonic-интерфейс ко всей экосистеме Rust.

## Установка

### Через PyPI
```bash
pip install nairobi-os
```

### Сборка из исходного кода
1. **Клонирование репозитория**:
    ```bash
    git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
    cd nairobi-connector-open-source
    ```

2. **Настройка виртуального окружения**:
    ```bash
    python3 -m venv .venv
    source .venv/bin/activate
    pip install maturin pyo3-build-config zbus anywidget traitlets pandas
    ```

3. **Сборка всего стека**:
    ```bash
    ./build_wheel.sh --release
    ```

## Использование

### Аналитика данных (In-Memory конвейер)
```python
import nairobi_os as nb

# Запуск фонового демона refinery
nb.connect()

# Импорт данных с использованием канала с нулевым копированием
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# Выполнение векторизованных расчетов в Rust refinery
profile = frame.crunch("value")
print(f"Среднее: {profile['mean']:.4f}")
print(f"Станд. откл.: {profile['std_dev']:.4f}")

# Выполнение произвольных SQL-запросов на отображенной в памяти таблице
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# Вызов интерактивного виджета визуализации, ускоренного Lagos
subset.plot(column="value")
```

### Использование компьютера (MCP сервер)
ИИ-агенты, использующие Nairobi Connector, должны следовать следующему основному циклу:
1. Выбрать целевое окно с помощью `nairobi_find_window`.
2. Получить текущую структуру интерфейса через `nairobi_get_ui_map`.
3. Считать тег TOON `[ID: N]` требуемого интерактивного элемента.
4. Выполнить действие на этом элементе с помощью `nairobi_interact` или `nairobi_type_text`.

## Настройка системы (руководство для разработчиков)

### Страницы памяти Huge Pages
Движок refinery отдает приоритет Huge Pages размером 1 ГБ для буферов с нулевым копированием. Чтобы включить их на хосте:
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*Примечание: Если страницы размером 1 ГБ недоступны, движок автоматически переключится на Transparent Huge Pages (THP).*

### io_uring и SQPOLL
Движок `DiracEngine` использует `io_uring` с опцией `SQPOLL` (ядерный поток опроса) для достижения максимальной пропускной способности ввода-вывода.

## Поддержка
Если вы находите Nairobi OS полезной, подумайте о поддержке проекта:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## Лицензия
Этот проект лицензирован на условиях лицензии **Apache License 2.0**.  
*(Примечание: Части формата TOON и реализации моста принадлежат авторам TOON).*

---
© 2026 Kevin Chege. All Rights Reserved.
