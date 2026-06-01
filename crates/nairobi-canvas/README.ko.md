[English](README.md) | [简体中文](README.zh-CN.md) | [廣東話](README.yue.md) | [Français](README.fr.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [Suomi](README.fi.md) | [Nederlands](README.nl.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Canvas: 즉시 모드 노드 그래프 시각적 컴파일러

Nairobi Canvas는 데이터 처리 파이프라인 구축을 위한 하드웨어 가속 시각적 컴파일러입니다. `egui`/`egui-snarl`을 기반으로 구축된 즉시 모드(immediate-mode) 노드 그래프 UI를 제공하며, 시각적 워크플로우를 Nairobi Hub에서 실행하기 위한 GVariant DAG(방향성 비순환 그래프) 형식으로 컴파일합니다.

## 주요 기능

- **시각적 파이프라인 빌더**: 데이터 워크플로우를 위한 드래그 앤 드롭 노드 그래프 인터페이스
- **네이티브 파일 선택기**: Ingest 노드의 📂 버튼을 클릭하여 CSV 파일 검색
- **SQL 쿼리 프리셋**: 사전 구성된 쿼리 템플릿 (모든 열, 단일 열, Where 절, 다중 열)
- **GVariant 직렬화**: 제로 카피 IPC를 위해 그래프를 GVariant 형식으로 컴파일
- **위상 정렬**: 자동 사이클 감지 및 실행 순서 지정

## 노드 유형

| 노드 | 입력 | 출력 | 설명 |
|------|--------|---------|-------------|
| **Ingest** | 0 | 1 | 네이티브 파일 선택기를 통해 CSV 데이터셋 로드 |
| **SqlQuery** | 1 | 1 | 입력 데이터에 대해 Polars SQL 쿼리 실행 |
| **AxiomCrunch** | 1 | 1 | 통계량 계산 (평균, 표준편차, 첨도) |
| **LagosPlot** | 1 | 0 | 시각화 렌더링 (스파크라인, 산점도, PNG, JPG) |

## 설치

```bash
pip install nairobi-os
```

또는 소스에서 빌드:
```bash
cargo build --release
# 캔버스 데모는 Rust 바이너리입니다 - examples/canvas_compile_demo.rs를 참조하세요.
```

## 사용법

### Rust (네이티브)

데모 애플리케이션 실행:
```bash
cargo run --example canvas_compile_demo
```

### Python

설치된 패키지 사용:
```python
import nairobi_os as nb

# DAG 컴파일을 위한 시각적 캔버스 열기
dag_bytes = nb.canvas.open()

# 컴파일된 파이프라인 실행
if dag_bytes:
    nb.canvas.execute(dag_bytes)
```

또는 전체 테스트 스크립트 실행:
```bash
python test_canvas.py
```

이 스크립트는 다음을 수행합니다:
1. `nairobi_os.ignite()` - Axum Refinery 및 Nairobi Hub 데몬 실행
2. `nb.canvas.open()` - 시각적 노드 그래프 에디터 실행
3. `nb.canvas.execute(dag_bytes)` - 타이밍 메트릭과 함께 컴파일된 파이프라인 실행

캔버스는 다음과 같이 사용할 수 있는 GVariant 인코딩된 DAG를 내보냅니다:
- `nb.canvas.execute()`를 통해 실행
- 나중에 사용하기 위해 디스크에 저장
- D-Bus/공유 메모리를 통해 전송

## 그래프 구축

1. 캔버스 그리드에서 **마우스 오른쪽 버튼**을 클릭하여 노드 메뉴 열기
2. 노드 유형 선택 (Ingest, SQL Query, Axiom Crunch 또는 Lagos Plot)
3. 출력 핀(파란색)에서 입력 핀(초록색)으로 드래그하여 노드 **연결**
4. **Compile Graph**를 클릭하여 워크플로우 직렬화

## 실행 흐름

```
캔버스 그래프 → GVariant DAG → Nairobi Hub → Axum Refinery / Lagos Vision
```

컴파일된 DAG는 IPC를 통해 Hub로 전송되며, Hub는 노드를 다음으로 라우팅합니다:
- **Axum Refinery**: 데이터 수집 및 통계 처리
- **Lagos Vision**: 하드웨어 가속 시각화 렌더링

아키텍처 세부 사항 및 전체 시스템 개요는 [메인 저장소 README](../README.md)를 참조하세요.

## 지원
Nairobi OS가 유용하다고 생각되면 프로젝트 지원을 고려해 주세요:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 라이선스
이 프로젝트는 **Apache License 2.0**에 따라 라이선스가 부여됩니다.

© 2026 Kevin Chege. All Rights Reserved.
