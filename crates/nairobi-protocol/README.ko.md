[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Protocol

## 개요
**Nairobi Protocol** 크레이트(crate)는 Nairobi OS 에코시스템 전반에서 사용되는 공유 D-Bus 인터페이스, GVariant 서명 및 데이터 구조를 정의합니다. Rust 코어, Hub 오케스트레이터 및 Python 바인딩 전반에서 강력한 타입 안전성을 제공하는 "단일 정보 기원(Source of Truth)" 역할을 수행합니다.

## 핵심 컴포넌트
- **인터페이스 정의**: 서비스 이름, 개체 경로 및 메서드 서명에 대한 상수값.
- **공유 타입**: `DistilledAnalytics` 및 `CorrelationResult`와 같은 GVariant 호환 구조체.
- **메모리 관리**: `memfd` 연산을 관리하는 `MemoryPipe` 래퍼 및 `iceoryx2` 아레나 정의.

## D-Bus 인터페이스
- **서비스 이름**: `org.nairobi.NairobiAxumRefinery1`
- **개체 경로**: `/org/nairobi/NairobiAxumRefinery1`
- **인터페이스**: `org.nairobi.NairobiAxumRefinery1`

## 사용 방법
Nairobi OS 에코시스템 내에서 통신이 필요한 모든 구성 요소의 의존성(dependency)으로 이 크레이트를 추가하여 사용하십시오.

## 개발상의 주의
이 크레이트를 수정할 때는 각별한 주의가 필요합니다. 수정 시 이를 참조하는 모든 크레이트를 다시 컴파일해야 하며, refinery 데몬과 Python 바인딩 간의 바이너리 호환성이 깨질 수 있습니다.

## 테스트
통합 테스트를 통해 GVariant 서명이 예상되는 D-Bus 프로토콜 사양과 완벽하게 일치하는지 확인합니다:
```bash
cargo test -p nairobi-protocol
```

## 지원
Nairobi OS가 유용하다고 생각되면 다음을 통해 저희 독립 시스템 연구를 지원해 주시기 바랍니다:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 라이선스
이 프로젝트는 **Apache License 2.0** 라이선스에 따라 사용 허가됩니다.

---
© 2026 Kevin Chege. All Rights Reserved.
