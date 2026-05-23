[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Hub

## 개요
**Nairobi Hub**는 Nairobi OS의 핵심 IPC(Inter-Process Communication, 프로세스 간 통신) 오케스트레이터입니다. 고성능 Rust 정제소(refinery) 데몬과 클라이언트 간의 파일 디스크립터, D-Bus 신호 및 공유 메모리 세그먼트의 조율을 총괄 관리합니다.

## 핵심 기능
- **파일 디스크립터(FD) 프록시**: GVariant 서명을 사용하여 D-Bus를 통해 `memfd` 파일 디스크립터를 안전하게 전달합니다.
- **서비스 관리**: `org.nairobi.NairobiAxumRefinery1` 서비스 수명 주기를 실시간으로 모니터링하고 관리합니다.
- **하이브리드 데이터 플레인**: 극대화된 성능을 위해 `iceoryx2` 공유 메모리를 통하거나, 호환성을 위해 D-Bus를 통해 데이터를 동적으로 라우팅합니다.
- **의미론적 디코딩**: 원시 바이너리 분석 데이터를 인간이 읽을 수 있는 보고서 및 네이티브 Python 구조체로 디코딩합니다.

## 아키텍처
Hub는 몇 가지 핵심 내부 모듈로 나뉩니다:
- `client.rs`: D-Bus 프록시 클라이언트.
- `shm_subscriber.rs`: `iceoryx2` 공유 메모리 구독 및 처리를 담당합니다.
- `decoder.rs`: GVariant 연산 결과를 Markdown 및 JSON으로 변환합니다.

## 사용 방법
Hub는 주로 `nairobi-python`에서 refinery 데몬과 연동하고 통신하기 위한 라이브러리로 백그라운드에서 사용됩니다.

## 개발상의 주의
Hub를 수정할 때 D-Bus 인터페이스 사양에 변경 사항이 생기면 반드시 `nairobi-protocol` 크레이트에도 해당 내용이 동기화되어 반영되었는지 확인하십시오.

## 테스트
Hub 통합 테스트를 통해 완벽한 IPC 왕복(round-trip) 작동 과정을 검증합니다:
```bash
cargo test -p nairobi-hub
```

## 지원
Nairobi OS가 유용하다고 생각되면 다음을 통해 저희 독립 시스템 연구를 지원해 주시기 바랍니다:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 라이선스
이 프로젝트는 **Apache License 2.0** 라이선스에 따라 사용 허가됩니다.

---
© 2026 Kevin Chege. All Rights Reserved.
