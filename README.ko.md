[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md) | [Finnish](README.fi.md) | [Cantonese](README.yue.md) | [Français](README.fr.md) | [Nederlands](README.nl.md)

# Nairobi OS: 고성능, 제로 카피 AI & 데이터 과학 인프라

[![PyPI Version](https://img.shields.io/pypi/v/nairobi-os.svg)](https://pypi.org/project/nairobi-os/)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
![System](https://img.shields.io/badge/Kernel-Linux_6.17_Native-orange.svg)
![Arch](https://img.shields.io/badge/Architecture-x86__64_/_ARM64-red.svg)

---

## 존재 이유

Nairobi OS는 Python의 기본 툴링이 각 레이어에서 유발하는 오버헤드를 지불하지 않고 로컬 하드웨어에서 AI 및 데이터 과학 파이프라인을 실행하기 위한 인프라입니다:

1. **Python 세금** — 데이터 집약적 워크로드에서의 엔드투엔드 메모리 복사, GIL 경합 및 인터프리터 오버헤드.
2. **브라우저 세금** — 장시간 실행되는 고빈도 상호작용을 위해 에이전트 도구가 브라우저 기반 인터페이스 위에 구축될 때 발생하는 렌더링 지연 및 통신 오버헤드.
3. **OS 커널 병목** — 데스크톱 자동화 워크로드에 지연을 가중시키는 프로세스 스케줄링 및 디스플레이 서버 오버헤드(Wayland vs. X11 컨텍스트 스위칭).

Nairobi OS는 Rust 기반 인프라 스택으로, 제로 카피 데이터 파이프라인, 하드웨어 네이티브 실행, 데스크톱 자동화를 위한 시맨틱(비픽셀) 인터페이스를 제공하여 이러한 오버헤드를 직접 제거하도록 구축되었습니다. 이 리포지토리는 해당 스택의 오픈 소스 코어입니다.

```
프로세서: AMD Ryzen 5 PRO 4650U (6코어, 12스레드)
그래픽: AMD Radeon RX Vega 6 내장 그래픽 (iGPU)
메모리: 32 GB RAM (높은 시스템 사용률 보유)
저장 장치: 256 GB NVMe (99% 가득 참)
```

바로 이 열악한 환경 속에서 저는 2025년 한 해 동안 외부와 차단된(air-gapped) 환경에서 지연 시간 제로로 작동하는 임상 의사 결정 지원 AI인 **Tumz**([Sarafakai](http://www.sarafakai.com))를 설계하고 개발했습니다. 이는 내장 그래픽(iGPU)에서 실시간 오디오 전사(transcription)와 임상 추론을 동시에 실행하며, 방대한 UMLS(Unified Medical Language System) 데이터 전체를 RAM에 상주시켜 동작합니다. 현재 우리는 케냐의 한 병원과 협력하여 Tumz의 1년 장기 임상 시험 파일럿을 진행 중입니다. 인간의 건강은 개발자의 어설픈 가정이 아니라, 철저하고 실증적인 검증을 요구하기 때문입니다.

Tumz를 개발하면서 저는 현대 데이터 과학 스택이 가지고 있는 거대하고 구조적인 비효율성을 온몸으로 마주했습니다.
1. **Python 세금**: 엔드투엔드 메모리 복사, GIL 병글넥(Bottleneck), 그리고 엄청난 런타임 오버헤드.
2. **브라우저 세금**: Manifest V3 복잡성, 렌더링 지연, 그리고 장시간 실행되는 자율 에이전트 대화 도중의 고빈도 통신 실패.
3. **OS 커널 병목**: 비효율적인 프로세스 스케줄링, CPU 스레드 기아 상태, 그리고 디스플레이 서버 오버헤드(Wayland vs. X11 컨텍스트 스위칭).

따라서 2025년 말, 저는 이러한 한계들을 완전히 우회하는 인프라 스택, 즉 제로 카피 데이터 파이프라인과 하드웨어 네이티브 AI 실행을 위해 설계된 에이전트 운영 체제(Agentic Operating System)를 구축하기 시작했습니다. 이 저장소는 그 엔진의 오픈 소스 핵심 코어입니다.

---

## 전투 기록: 지난 1년간 9,180건의 기여

현대 개발 커뮤니티의 어떤 비판가들은 새롭고 고도의 프로젝트들을 바라보며 이를 "AI가 생성한 템플릿 코드"라 부릅니다. 그런 회의론자들에게는 커밋 로그라는 충분하고 직접적인 증거를 제시합니다.

저의 또 다른 GitHub 프로필(https://github.com/ChegeKenya)은 매일 매일의 시스템 엔지니어링에 대한 객관적인 기록입니다. 2025년 한 해 동안 저는 7,888건의 기여를 기록했습니다. 2026년 초 5개월 동안 추가로 1,420건을 더했습니다. 지난 365일간의 기여 건수는 9,180건에 달합니다. 저지대의 녹색 커밋은 low-latency Rust 런타임, clinical AI 파이프라인, 제로 카피 공유 메모리 시스템을 가로지르는 거의 간단 없는 연속입니다. 이 코드는 전장에서 쓰여졌고 베어 메탈에서 컴파일되었며 바이트 단위로 검토되었습니다.

```
2025: [██████████████████████████████████████████████████] 7,888 Commits
  2026: [██████████] 1,420 Commits
  Total (Last Year): 9,180 Commits of Pure Systems Code
```

### 하드웨어 네이티브 개발 제약

저의 프로그래밍 여정은 하드웨어 수준의 시스템 구조와 극한의 기계적 최적화에 기반합니다. 2015년, 저는 [케냐 실리콘밸리에 관한 논문](https://www.linkedin.com/pulse/building-kenyas-silicon-valley-making-work-kevin-chege/)에서 아프리카 대륙의 분산된 고도 기술 구축 계횚을 공유했습니다. LLM 전환은 2023년에 시작되었고, 저는 초기 [2023년 LLM 래퍼 데모](https://www.linkedin.com/feed/update/urn:li:activity:7102930955807449088/)에서 기록된 것처럼 고수준 래퍼의 구조적 한계를 일찍 깨달았습니다.

기포 네트워크 위에 고수준 래퍼를 쌓는 것은 구조적인 막다른 길임을 깨달았습니다. 진정한 전쟁은 로컬 하드웨어의 제약과 직접적인 자원 할당의 교차점에서 벌어집니다.

2025년과 2026년 전체에 걸쳐 저는 극히 제한된 Lenovo X13 ThinkPad에서 생활과 개발을 이어갔습니다.

```
프로세서: AMD Ryzen 5 PRO 4650U (6코어, 12스레드)
그래픽: AMD Radeon RX Vega 6 iGPU (Shared Memory Architecture)
메모리: 29 GiB RAM (높은 시스템 사용률)
저장 장치: 256 GB NVMe (99% 가득 차고 자원 매우 제한됨)
호스트 커널: Linux 6.17.0-29-generic
```

바로 이 머신에서 저는 2025년 **Tumz**([Sarafakai](http://www.sarafakai.com))를 구축했습니다. 이것은 공기 틈(air-gapped) 환경에 완충되고 지연 시간 제로의 임상 의사 결정 지원 AI입니다. 통합 GPU(iGPU)에서 실시간 음성 전사와 임상 추론을 동시에 실행하며 Unified Medical Language System (UMLS) 사전 전체를 공유 RAM에 상주시켰습니다. Sarafakai는 현재 케냐 병원과의 1년 간의 임상 시험 파일럿과 함께 있습니다. 인간의 건강은 개발자의 추측이 아니라 철저하고 실증적 검증을 필요로 합니다.

---

## 글로벌 확장 및 텔레메트리

2026년 5월 6일 출시된 Nairobi OS는 전 세계의 시스템 프로그래머, 퀀트 연구원 및 에지 컴퓨팅 아키텍트들 사이에서 빠르게 자리를 잡아가고 있습니다. 이러한 다운로드 통계는 실시간 [ClickPy Nairobi OS 대시보드](https://clickpy.clickhouse.com/dashboard/nairobi-os)에서 제공되며, 여기서 직접 지표를 검색하고 탐색할 수 있습니다.

### 글로벌 누적 분포 (2026년 5월 6일 – 2026년 5월 23일)

| 지표 | 측정값 | 컨텍스트 |
| :--- | :--- | :--- |
| **글로벌 순위** | **#75,293** | PyPI의 활성 797,894개 패키지 중 |
| **백분위** | **9.43%** | 시스템 레벨 Python 확장 중 최상위 등급 |
| **총 다운로드 수** | **1,525** | 깔끔하고 유기적이며 강력한 도입 의사를 가진 개발자들의 다운로드 |

### 버전별 다운로드 볼륨

```
  0.2.0 [████████████████████████████████████████] 342
  0.2.1 [██████████████████████████] 224
  0.3.0 [████████████████████████] 212
  0.3.1 [████████████████████] 176
  0.1.0 [███████████████████] 169
  0.4.1 [██████████████] 120
```

### 도입량 기준 상위 10대 국가/지역

| 순위 | 지역 | 국가 코드 | 다운로드 볼륨 |
| :--- | :--- | :--- | :--- |
| 1 | 미국 | US | 661 |
| 2 | 홍콩 | HK | 103 |
| 3 | 중국 | CN | 84 |
| 4 | 독일 | DE | 74 |
| 5 | 일본 | JP | 65 |
| 6 | 싱가포르 | SG | 56 |
| 7 | 영국 | GB | 51 |
| 8 | 프랑스 | FR | 51 |
| 9 | 러시아 | RU | 42 |
| 10 | 대한민국 | KR | 30 |

---

## 지원 및 주권

Nairobi OS가 귀하의 데이터 파이프라인을 최적화하고, 클라우드 비용을 절감하며, 로컬 에이전트 아키텍처를 가속화하고 있다면, 우리의 독립적인 시스템 연구를 지원하는 것을 고려해 보십시오. 모든 기부금은 나이로비 현지 하드웨어 수준의 컴파일러 최적화와 에지 컴퓨팅 테스트 환경 구축에 직접 투입됩니다.

[![Nairobi OS 개발 지원하기](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

---

## 핵심 기능

* **픽셀 없는 컴퓨터 사용**: 느리고 비용이 많이 드는 비전 기반 에이전트 파이프라인을 우회합니다. AT-SPI2 및 TOON(Token-Oriented Object Notation) 압축 알고리즘을 통해 Linux 데스크톱과 네이티브하게 상호작용하고, 원시 계층 구조 트리를 LLM에 직접 입력합니다.
* **제로 카피 수집**: `io_uring` 및 1GB Huge Pages를 사용하는 하드웨어 가속, 커널 바이패스 데이터 로딩.
* **하드웨어 가속 시각화**: `wgpu` 및 `egui`를 기반으로 빌드된 `lagos-lite` 렌더링 데몬을 이용한 지연 시간 제로 수준의 대화형 Jupyter 플로팅.
* **벡터화된 분석 실행**: Polars 쿼리 실행 및 Rayon 다중 스레드 데이터 파이프라인을 사용한 극단적인 CPU 활용율 극대화.
* **소브린(주권) 인터페이스**: 메모리 맵핑 파일 디스크립터 및 IPC를 캡슐화하는 매끄러운 Python API (`SovereignFrame`).

---

## 오픈 소스 vs. 엔터프라이즈 아키텍처

Nairobi OS는 구조적으로 이원화되어 있습니다. 오픈 소스 저장소는 기본적인 고성능 데이터 처리 및 단일 노드 시각화 프리미티브를 제공합니다. 비공개 상용 에코시스템에는 고급 다중 에이전트, 고가용성 및 산업별 솔루션 구현이 포함되어 있습니다.

```
                                  +---------------------------------------+
                                  |         Nairobi Python API            |
                                  +---------------------------------------+
                                                      |
                                     [ D-Bus / 공유 메모리 상의 GVariant ]
                                                      |
                                                      v
                                  +---------------------------------------+
                                  |           Nairobi Hub                 |
                                  +---------------------------------------+
                                                      |
                    +---------------------------------+---------------------------------+
                    |                                                                   |
                    v                                                                   v
     +------------------------------+                                    +------------------------------+
     |     Axum Refinery (Data)     | <===[ 제로 카피 IPC / iceoryx2 ]===> |     Lagos Vision (Visual)    |
     +------------------------------+                                    +------------------------------+
```

### Open Source Crate Workspace (`crates/`)
1. `nairobi-axum-refinery` — Rust daemon managing raw data ingestion, Rayon-parallelized statistics, and Polars-vectorized query execution.
2. `nairobi-hub` — Central IPC orchestrator; routes file descriptors and signals between clients and the refinery daemon.
3. `lagos-lite` — Local/headless rendering engine using egui/wgpu hardware acceleration with zero-copy mmap data access.
4. `nairobi-protocol` — Shared protocol layer: GVariant serialization schemes, error types, and shared-memory layouts.
5. `nairobi-python` — The Python extension module, compiled via PyO3 and packaged with Maturin (`nairobi-os`).
6. `nairobi-canvas` — Immediate-mode node-graph compiler with hardware-accelerated UI (wgpu/egui), including a native file picker and SQL query presets.
7. `nairobi-connector` — Model Context Protocol (MCP) server and AT-SPI2 semantic accessibility bridge exposing TOON representations for LLM agents.
### 비공개 엔터프라이즈 에코시스템 (`modules/`)

엔터프라이즈 등급의 핵심 컴포넌트들은 별도의 비공개 저장소(`Sovereign-Systems-Lab`)에 보관되며 산업, 금융 및 국가 인프라 시스템용으로 라이선스가 제공됩니다.

1. **`sovereign-ui`**: 엔터프라이즈 등급의 AT-SPI2 엔진. Aegis 프로토콜 보안, 하드웨어 바인딩 및 프로덕션 수준의 데스크톱 제어를 구현합니다.
3. **`tactical-rtos-node`**: 안전이 핵심적인 에지 산업 자동화를 위한 초저지연 실시간 운영 체제(RTOS) 스케줄러.
4. **`industrial-guardian-rust` / `industrial-guardian-python`**: 예측적인 OOM, 메모리 누수 및 시스템 충돌 회피 기능을 포함한 자율적 사이트 신뢰성 공학(SRE) 레이어.
5. **`fintech-bridge-rust`**: 실시간 고빈도 거래 파서 및 레거시 메인프레임 브리지(EBCDIC/SBA 터미널 파싱).
6. **`aviation-audio-rust`**: 서브 밀리초 수준의 락 프리(lock-free) 오디오 스트림 처리, 음향 텔레메트리 분석 및 원시 파형 DSP.
7. **`drawbridge_api`**: 신뢰할 수 없는 클라우드 에이전트의 호출로부터 로컬 커널을 격리하는 안전하고 인증된 다중 테넌트 gRPC 드로우브리지.

### 기능 비교 매트릭스

| 능력 / 기능 | 오픈 소스 코어 (`crates/`) | 엔터프라이즈 스위트 (`modules/`) |
| :--- | :---: | :---: |
| **수집 엔진** | `mmap` / `copy_file_range` | `io_uring` + `SQPOLL` + 1GB Huge Pages |
| **통계 분석** | 기본 기술 통계 | 벡터화된 다중 패스 왜도/첨도, 상관관계 분석 |
| **쿼리 엔진** | 프로세스 내 Polars SQL | 분산형 Apache Arrow / DataFusion 클러스터 |
| **IPC 메커니즘** | POSIX 공유 메모리 / D-Bus | 제로 카피 `iceoryx2` 공유 메모리 아레나 |
| **시각화** | 로컬 Jupyter `anywidget` | WebRTC GStreamer / 투명 Wayland Layer-Shell 오버레이 |
| **보안 및 규정 준수** | 표준 POSIX 경계 | Aegis 프로토콜, SHA-256 체인 포렌식 원장 |
| **인증** | 없음 (로컬 신뢰 사용자) | 하드웨어 바인딩 (TPM 2.0 / CPU ID), 프라이빗 PKI |
| **플랫폼 대상** | 단일 노드 Linux | 분산 클라우드 / 에지 노드 / 고빈도 거래 |

---

## 설치 및 설정

### 요구 사항
- **OS**: Linux (Ubuntu 22.04 이상 권장) 또는 Windows Subsystem for Linux (WSL2).
- **GPU**: Vulkan, Metal 또는 OpenGL 호환 드라이버.
- **Python**: 3.10 이상.
- **Rust**: 안정적인 툴체인 (소스로 빌드하는 경우).

### 빠른 설치 (PyPI)
```bash
pip install nairobi-os
```

### 소스 빌드
네이티브 데몬과 Python 확장을 포함한 작업 공간 전체를 컴파일하려면:

1. **저장소 복제**:
   ```bash
   git clone https://github.com/KevinKenya/nairobi-connector-open-source.git
   cd nairobi-connector-open-source
   ```

2. **가상 환경 구성**:
   ```bash
   python3 -m venv .venv
   source .venv/bin/activate
   pip install maturin pyo3-build-config zbus anywidget traitlets pandas
   ```

3. **작업 공간 빌드 실행**:
   ```bash
   chmod +x build_wheel.sh
   ./build_wheel.sh --release
   ```
   이 과정은 네이티브 데몬들을 컴파일하여 패키지 디렉토리로 복사하고, `crates/nairobi-python/target/wheels/` 아래에 wheel 파일을 빌드합니다.

---

## 사용 가이드

### 1. 데이터 분석 (인메모리 파이프라인)

Nairobi OS는 `SovereignFrame` API를 제공합니다. 이 API는 내부적으로 원시 메모리 맵핑을 매끄럽게 처리하여 고속의 데이터 처리를 가능하게 만듭니다.

```python
import nairobi_os as nb

# 백그라운드 정제소(refinery) 데몬 시작
nb.connect()

# 제로 카피 메모리 파이프라인을 사용하여 데이터세트 수집
frame = nb.read_csv("simulator/fndds_ingredient_nutrient_value.csv")

# Rust refinery를 통한 벡터화된 연산 실행
profile = frame.crunch("value")
print(f"Mean: {profile['mean']:.4f}")
print(f"Std Dev: {profile['std_dev']:.4f}")

# 메모리 맵핑된 frame 상에서 직접 임의의 SQL 쿼리 실행
subset = frame.query("SELECT * FROM dataset WHERE value > 50.0")

# Lagos 가속 대화형 플로팅 위젯 호출
subset.plot(column="value")
```

### 2. 픽셀 없는 컴퓨터 사용 (MCP)

AT-SPI2 의미론적 인터페이스를 사용하려면 AI 에이전트가 화면 캡처 이미지를 읽는 대신 아래와 같이 노출된 MCP 서버 도구들과 상호작용해야 합니다.

```
                    컴퓨터 사용 시퀀스
                     
  [ LLM 에이전트 ]                                [ Nairobi OS ]
        |                                             |
        |===> nairobi_find_window("Text Editor") ====>| (대상 탐색)
        |<=== 윈도우 ID 및 영역 경계 반환 =============|
        |                                             |
        |===> nairobi_get_ui_map() ==================>| (TOON 생성)
        |<=== 압축된 Markdown 트리 반환 ==============|
        |     "[ID: 12] Button: 'Save'"               |
        |                                             |
        |===> nairobi_interact(12, "click") =========>| (동작 실행)
        |<=== 성공 상태 코드 반환 =====================|
```

---

## 시스템 튜닝 (기여자 가이드)

성능 벤치마크에 제시된 우수한 속도를 경험하려면 호스트 커널이 시스템 레벨 메모리 맵핑을 지원하도록 구성되어야 합니다.

### 1GB Huge Pages
Nairobi OS는 대규모 데이터세트 처리 시 발생하는 CPU의 TLB(Translation Lookaside Buffer) 변환 오버헤드를 우회하기 위해 1GB Huge Pages를 사용합니다.

Linux 호스트에서 Huge Page를 할당하려면 아래와 같이 실행하십시오.
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*참고: 메모리 파편화 등의 원인으로 시스템이 1GB 페이지를 즉시 할당하지 못하는 경우, 엔진은 자동으로 THP(Transparent Huge Pages)로 대체하여 작동합니다.*

### D-Bus Broker 설정
고빈도 신호 처리가 빈번한 프로덕션 환경의 경우, 기존의 느린 `dbus-daemon` 대신 가볍고 빠른 `dbus-broker`를 설치하여 커뮤니케이션 제어판의 신호 전파 속도를 극대화할 것을 적극 권장합니다.

---

## 라이선스

이 프로젝트는 **Apache License 2.0**에 따라 라이선스가 부여됩니다.  
*(참고: TOON 포맷 및 브리지 구현의 일부는 TOON 원작자들에게 공헌이 귀속됩니다.)*

---
© 2026 Kevin Chege. All Rights Reserved.  
*Sovereign Systems Lab, Nairobi, Kenya.*
