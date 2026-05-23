[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Lagos Vision (lagos-lite)

## 개요
**Lagos Vision**은 Nairobi OS의 초고성능 그래픽 시각화 렌더링 엔진입니다. 분석 대상 데이터를 `memfd` 파일 디스크립터에서 GPU 그래픽 파이프라인으로 직접 제로 카피 메모리 맵핑하여 수백만 개의 데이터 포인트를 서브 밀리초 수준의 초저지연 시간으로 화면에 시각화하도록 설계되었습니다. Lagos는 WebSockets 프로토콜을 통하여 Jupyter 노트북 위젯에 JPEG로 고속 인코딩된 프레임 이미지 스트림을 전송하는 헤드리스(headless) 백그라운드 데몬 프로세스로 작동합니다.

## 핵심 기능
- **제로 카피 렌더링**: 대량의 원시 데이터가 `memfd` 파일 디스크립터에서 `wgpu` 그래픽 버퍼로 복사 과정 전혀 없이 다이렉트로 메모리 맵핑됩니다.
- **하드웨어 가속**: 고성능 플로팅 처리를 위해 `egui` 및 `wgpu` (Vulkan, Metal, DX12 또는 OpenGL) 모듈을 전적으로 활용합니다.
- **GPU 상의 LTTB 다운샘플링**: 대규모 데이터세트를 그릴 때 시각적 왜곡 및 누락을 방지하고 물리적 정확도를 완벽 보존하기 위해 GPU 계층에서 LTTB (Largest-Triangle-Three-Buckets) 알고리즘을 하드웨어 가속 실행합니다.
- **이벤트 기반 아키텍처**: 시스템이 유휴(idle) 상태일 때는 CPU 점유율을 0%로 유지하며, 오직 데이터 갱신 및 사용자 입력 인터랙션이 발생할 때만 즉각 렌더링 루틴을 가동합니다.

## 아키텍처
Lagos Vision 엔진은 다음 핵심 요소로 나뉩니다:
- **Lagos Lite**: 렌더링 파이프라인 엔진을 정의하는 코어 내부 라이브러리.
- **Lagos Vision Daemon**: `wgpu` 드라이버 인스턴스 및 WebSocket 통신 서버를 관리하는 실제 바이너리 구동 프로세스.
- **Lagos Widget**: 데이터 프레임 이미지 스트림을 브라우저에 표시하는 Python `anywidget` 웹 프론트 구성 요소.

## 설치 및 설정

### 필수 요구 사항
- **GPU**: Vulkan 호환 고성능 GPU (또는 가상 환경 등 물리 디스플레이 드라이버가 부재한 곳을 위한 OSMesa 소프트웨어 렌더링 에뮬레이터).
- **시스템 라이브러리**: `libosmesa6-dev`, `mesa-utils`, `xvfb`.

### 빌드
```bash
cargo build --release -p lagos-lite --bin lagos-vision-daemon
```

## 사용 가이드

### Nairobi OS 스택 연동 사용 시
Lagos 엔진의 작동 인터페이스는 주로 Python 프레임 객체의 `SovereignFrame.plot()` 호출을 통해 내부적으로 완벽하게 추상화되어 투명하게 자동 작동합니다.

### 독립 환경에서의 수동 디버깅 테스트 방법
데이터 렌더링 파이프라인의 단독 작동을 수동 검증해 보려면 다음과 같이 데몬 프로세스를 명령어로 직접 기동해 볼 수 있습니다:
```bash
./target/release/lagos-vision-daemon --fd <FD_INT> --width 1000 --height 400
```

## 개발 및 커스텀 가이드

### 사용자 정의 시각화 셰이더 레이어 구현 방법
1.  **셰이더 파이프라인 수정**: `src/pipeline.rs` 파일 내에서 사용자 정의 WGSL 정점(vertex) 및 픽셀/프래그먼트(fragment) 셰이더 코드를 작성 및 수정합니다.
2.  **버퍼 구조 업데이트**: 입력된 `memfd` 메모리 주소를 새로 추가할 셰이더의 바인드 그룹(bind groups)과 매칭합니다.
3.  **UI 컨트롤 제어 연동**: `src/device.rs` 소스 파일 내의 `egui` 구성 계층에 분석 제어를 위한 슬라이더바, 버튼 등 제어 요소를 확장 추가합니다.

### 물리 디스플레이가 없는 헤드리스 클라우드 환경에서의 운영 방법
구글 코랩(Google Colab) 등 물리 모니터 장치가 연동되어 있지 않은 완전 가상 클라우드 컴퓨팅 환경에서는 Lagos가 `xvfb-run` 및 OSMesa 백엔드를 투입하여 에뮬레이션 가동합니다:
```bash
xvfb-run -s "-screen 0 1024x768x24" ./target/release/lagos-vision-daemon ...
```

## 테스트
Lagos 소스 트리에는 렌더링된 결과 이미지를 캡처하여 기준치 이미지(golden images)와 비교 검증하는 강력한 시각화 통합 테스트 도구가 내장되어 있습니다:
```bash
cargo test -p lagos-lite
```

## 문제 해결
- **WebSocket 연동 에러**: Colab 또는 SageMaker 등의 가상 웹 서비스 환경에서 외부 프록시 포트가 차단 없이 정상 맵핑 연동되어 있는지 점검하십시오. 구글 코랩 환경이 식별되는 경우 Nairobi Python이 자동으로 필요한 주소 맵핑을 탐지하고 우회 연동합니다.
- **WGPU 어댑터 에러**: 대상 컴퓨터의 GPU 드라이버 설치가 완료되었는지 확인하십시오. 만약 물리 GPU 하드웨어가 전혀 존재하지 않는 완전 CPU 전용 서버 머신인 경우 Lagos가 자동으로 소프트웨어 가속 어댑터(software adapter)로의 대체를 테스트 시도합니다.

## 지원
Nairobi OS가 유용하다고 생각되면 다음을 통해 저희 독립 시스템 연구를 지원해 주시기 바랍니다:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 라이선스
이 프로젝트는 **Apache License 2.0** 라이선스에 따라 사용 허가됩니다.

---
© 2026 Kevin Chege. All Rights Reserved.
