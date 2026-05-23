[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Axum Refinery

## 개요
**Axum Refinery**는 Nairobi OS의 초고성능 연산 코어 엔진입니다. Rust로 작성되었으며, 커널 바이패스 I/O 및 벡터화된 병렬 분석 기법을 통해 최신 하드웨어 리소스를 극한까지 활용하여 포화 상태로 만들도록 설계되었습니다. 익명 메모리 파일 디스크립터(`memfd`)에 적재된 데이터의 전체 수명 주기를 관리하는 D-Bus 백그라운드 서비스로 작동합니다.

## 핵심 기능
- **Dirac 수집 엔진**: `io_uring` (Tier 1), `copy_file_range` (Tier 2) 및 `mmap` (Tier 3) 단계를 활용하는 3단계 하드웨어 가속 데이터 수집 전략.
- **Axiom Crunch**: Polars 및 Rayon 멀티스레드 기반의 고속 벡터화 통계 모멘트 계산 (평균, 분산, 왜도, 첨도).
- **Relational Strike**: 극대화하여 최적화된 피어슨 및 스피어만 상관 계수 연산 처리.
- **SQL 분석**: `polars-sql`을 통해 메모리에 상주하고 있는 데이터에 대해 직접 임의의 SQL 쿼리 고속 실행.
- **제로 카피 데이터 플레인**: 분석 처리 결과를 복사 없이 `iceoryx2` 공유 메모리 아레나 및 D-Bus 채ネル을 통해 초고속 배포.

## 아키텍처
정제소(refinery) 데몬은 특화된 개별 하위 엔진 구조로 나뉩니다:
- `DiracEngine`: 하드웨어 가속 I/O 처리를 담당합니다.
- `AnalyzeEngine`: 통계 분석 연산 및 SQL 실행을 처리합니다.
- `DbusService`: `org.nairobi.NairobiAxumRefinery1` D-Bus 인터페이스 사양을 구현합니다.

## 설치 및 설정

### 필수 요구 사항
- **OS 커널**: Linux 5.10 이상 (WSL2 환경 지원).
- **의존성**: `libdbus-1-dev`, `pkg-config`.
- **Huge Pages**: 최고의 성능을 확보하려면 1GB Huge Pages 설정을 필히 활성화하십시오.
    ```bash
    echo 1 | sudo tee /sys/kernel/mm/hugepages/hugepages-1048576kB/nr_hugepages
    ```

### 빌드
```bash
cargo build --release -p nairobi-axum-refinery
```

## 개발 및 기여 가이드

### 커널 레벨 시스템 설정
`DiracEngine`은 최고 효율의 비동기 I/O를 위해 `IORING_SETUP_SQPOLL`을 사용하려고 시도합니다. root 권한 없이 일반 사용자로 작동하게 하려면, `/proc/sys/kernel/unprivileged_userns_clone` 값을 임의 조정하거나 `CAP_SYS_ADMIN` 권한을 프로세스에 일시 부여하여 실행해야 할 수 있습니다.

### 새로운 통계 분석 지표 추가 방법
1.  **지표 연산 정의**: `src/analyze.rs` 소스 파일 내에서 `StatisticalProfile` 구조체 및 해당 `compute` 구현 메서드에 관련 연산을 추가합니다.
2.  **프로토콜 동기화**: `crates/nairobi-protocol/src/types.rs` 파일의 `DistilledAnalytics` 구조체에 새로 추가할 필드 속성을 선언합니다.
3.  **D-Bus 외부 노출**: `src/dbus_service.rs` 내의 D-Bus 서비스 계층이 업데이트된 분석 프로필 구조를 올바르게 직렬화하는지 확인합니다.

### 테스트 실행
Refinery 엔진은 비동기 통합 테스트를 위해 `tokio::test` 프레임워크를 적극 활용합니다:
```bash
cargo test -p nairobi-axum-refinery
```

#### 격리된 환경에서의 독립 Mocking 테스트 방법
D-Bus IPC 계층을 거치지 않고, 다음과 같이 수동으로 `memfd`를 직접 임의 생성하여 `AnalyzeEngine` 기능을 가볍게 독립적으로 테스트해 볼 수 있습니다:
```rust
let opts = memfd::MemfdOptions::default();
let mfd = opts.create("test.csv")?;
// 임의의 테스트 데이터 쓰기 처리...
let engine = AnalyzeEngine::new()?;
let results = engine.analyze(mfd.into_fd(), "target_column")?;
```

## 문제 해결
- **`io_uring` 초기화 실패**: 현재 사용 중인 OS 커널이 `io_uring` 기능 사양을 정상적으로 빌드 지원하는지 점검하십시오 (`zgrep CONFIG_IO_URING /proc/config.gz`).
- **Huge Page 할당 에러**: 호스트 머신에 물리적이고 연속적인 자유 메모리 영역이 충분히 남아 있는지 진단하십시오 (`grep Huge /proc/meminfo`).

## 지원
Nairobi OS가 유용하다고 생각되면 다음을 통해 저희 독립 시스템 연구를 지원해 주시기 바랍니다:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 라이선스
이 프로젝트는 **Apache License 2.0** 라이선스에 따라 사용 허가됩니다.

---
© 2026 Kevin Chege. All Rights Reserved.
