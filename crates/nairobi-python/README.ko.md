[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi OS (nairobi-python)

## 개요
**Nairobi OS**는 극대화된 자원 효율성을 위해 설계된 고성능, 분산 AI 및 데이터 과학 인프라스트럭처입니다. 특화된 Rust 기반의 정제소(refinery) 백그라운드 데몬 프로세스를 통합 연동함으로써, 자원 및 물리 환경이 매우 열악하고 제한적인 곳(에지, IoT, Serverless)에서도 초대형 데이터세트를 원활히 가공 처리할 수 있도록 지원하며, MCP 규격을 준수하는 접근성 브리지를 통해 **"픽셀 없는 컴퓨터 사용(Computer Use without pixels)"**을 지원합니다.

`io_uring`, `memfd`, Huge Pages와 같은 최신 리눅스 커널 수준의 내부 기능들을 활용함으로써 Nairobi OS는 서브 밀리초 수준의 IPC 오버헤드와 제로 카피 데이터 처리 파이프라인을 실현합니다.

## 핵심 기능
- **픽셀 없는 컴퓨터 사용**: AT-SPI2 및 TOON(Token-Oriented Object Notation) 압축 알고리즘을 통해 리눅스 데스크톱 환경과 직접 상호작용하여 AI 에이전트 구동 시 스크린샷 이미지 분석, OCR 문자 판독 또는 불필요한 시각적 처리 과정을 완전히 생략합니다.
- **제로 카피 데이터 수집**: `io_uring` 및 1GB Huge Pages를 사용하는 하드웨어 가속 기법으로 원시 데이터를 복사 과정 전혀 없이 초고속 로딩합니다.
- **하드웨어 가속 시각화**: Lagos Vision 렌더링 엔진(`wgpu` 및 `egui`)을 연동한 대화형 Jupyter 플로팅 시각화를 초저지연 시간으로 제공합니다.
- **융합형 분석 파이프라인**: 단 한 번의 D-Bus IPC 왕복(round-trip) 연산을 통해 데이터 수집, 통계 계산 및 상관관계 산출을 한꺼번에 고속 처리합니다.
- **커널 바이패스 성능**: Polars 및 Rayon 멀티스레드 아키텍처를 전적으로 투입하여 하드웨어 성능 한계점까지 벡터화된 연산을 고속 실행합니다.
- **소브린(주권) 인터페이스**: 내부적인 저수준 IPC 처리 및 메모리 관리의 난해함을 지우고 간결한 객체 지향형 추상화를 지원하는 Python API (`SovereignFrame`) 제공.

## 아키텍처
Nairobi OS는 D-Bus 및 공유 메모리(Shared Memory) 통신으로 연결된 3개의 전문화된 구성 요소로 구축되어 있습니다:
1. **Nairobi Axum Refinery**: 고성능 Rust 코어 엔진. 원시 데이터 수집 및 병렬 분석 처리를 전담합니다.
2. **Nairobi Hub**: IPC 오케스트레이터. refinery 데몬과 Python 클라이언트 간의 파일 디스크립터 및 통신 신호를 중재 및 라우팅합니다.
3. **Lagos Vision**: 시각 피질. `memfd` 메모리를 직접 GPU 파이프라인에 맵핑하는 헤드리스 이벤트 기반 렌더링 엔진.
4. **Nairobi Connector**: 의미론적 브리지. 리눅스 데스크톱의 접근성 트리를 LLM에 연동 노출하는 MCP 서버.
5. **Nairobi Python**: 고수준 인터페이스. Rust 에코시스템 스택 전체를 Python에서 편리하게 다룰 수 있는 Pythonic 인터페이스를 완벽 제공합니다.

## 설치

### PyPI 설치
```bash
pip install nairobi-os
```

### 소스 직접 빌드
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

3. **전체 스택 빌드 실행**:
    ```bash
    ./build_wheel.sh --release
    ```

## 사용 가이드

### 데이터 분석 (인메모리 파이프라인)
```python
import nairobi_os as nb

# 백그라운드 정제소(refinery) 데몬 시작 및 연동
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

### 컴퓨터 사용 (MCP 서버)
Nairobi Connector를 사용하는 AI 에이전트는 다음 기본 루프를 따릅니다:
1. `nairobi_find_window`를 사용하여 대상 창을 지정합니다.
2. `nairobi_get_ui_map`을 통해 현재 데스크톱 접근성 지도를 확인합니다.
3. 대상 상호작용 요소의 TOON `[ID: N]`을 읽어들입니다.
4. `nairobi_interact` 또는 `nairobi_type_text`를 호출하여 해당 요소에 대해 아토믹 동작을 가동합니다.

## 시스템 구성 (기여자 가이드)

### Huge Pages 설정
Refinery 데몬은 제로 카피 버퍼의 극대화된 효율성을 확보하기 위해 1GB Huge Pages를 최우선으로 투입합니다. 이를 활성화하려면 리눅스 호스트에서 다음을 기동하십시오:
```bash
echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
```
*참고: 1GB 물리 페이지 할당이 불가능한 시스템 환경인 경우, 엔진이 자동으로 THP(Transparent Huge Pages) 백엔드를 시도합니다.*

### io_uring 및 SQPOLL
`DiracEngine`은 최고의 I/O 처리량을 유지하기 위해 리눅스 커널의 SQPOLL(폴링 스레드)을 포함하는 `io_uring`을 사용합니다.

## 지원
Nairobi OS가 유용하다고 생각되면 다음을 통해 저희 독립 시스템 연구를 지원해 주시기 바랍니다:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 라이선스
이 프로젝트는 **Apache License 2.0** 라이선스에 따라 사용 허가됩니다.  
*(참고: TOON 포맷 및 브리지 구현의 일부는 TOON 원작자들에게 저작권 공헌이 귀속됩니다.)*

---
© 2026 Kevin Chege. All Rights Reserved.
