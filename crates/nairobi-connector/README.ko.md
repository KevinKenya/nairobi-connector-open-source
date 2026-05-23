[English](README.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Deutsch](README.de.md) | [日本語](README.ja.md) | [Русский](README.ru.md) | [한국어](README.ko.md)

# Nairobi Connector

## 개요
**Nairobi Connector**는 Nairobi OS를 위한 AT-SPI2 의미론적 브리지이자 MCP(Model Context Protocol) 서버입니다. Linux 데스크톱의 접근성 트리를 TOON(Token-Oriented Object Notation)이라는 초고밀도 토큰 최적화 형식으로 변환하여 LLM 및 AI 에이전트에 노출함으로써 **"픽셀 없는 컴퓨터 사용(Computer Use without pixels)"**을 지원합니다. 픽셀 대신 순수하게 의미론적인 UI 노드를 기반으로 동작하여 거의 즉각적인 동작 실행 속도를 달성하고 토큰 소모를 극적으로 줄입니다.

## 핵심 기능
- **픽셀 없는 컴퓨터 사용**: AT-SPI2를 통해 Linux 데스크톱과 직접 상호작용하여 스크린샷, OCR 또는 시각적 처리의 필요성을 완전히 배제합니다.
- **TOON 압축 알고리즘**: 원시 D-Bus 접근성 트리를 고도로 압축된 Markdown 표현으로 변환합니다. 비대화형 "노이즈" 노드를 필터링하고 상호작용 가능한 요소에 순차적인 ID를 할당하여, 50ms 미만의 작동 속도에서 500 토큰 미만의 출력을 냅니다.
- **MCP 서버 통합**: 호환되는 LLM 에이전트에 의미론적 도구를 네이티브하게 제공하는 견고한 `rmcp` 기반 서버를 구현합니다.
- **안전한 세션 수명 주기**: `stdio` 파이프가 끊어지는 경우 `RegistryLock`을 자동으로 해제하여 OS가 멈추는 현상을 방지하는 하트비트 모니터를 갖추고 있습니다.

## 아키텍처
커넥터는 LLM(MCP 경유)과 Linux 데스크톱(AT-SPI2/D-Bus 경유) 간의 양방향 브리지 역할을 수행합니다. 이는 `NeuralSession` 레이어를 캡슐화하여 창 탐색, UI 트리 순회 및 로컬 동작 주입을 관리합니다.

### 제공되는 MCP 도구
- `nairobi_find_window`: 대소문자 구분 없이 창 제목 하위 문자열을 기반으로 대상 창을 찾아 지정합니다.
- `nairobi_get_ui_map`: 현재 UI 접근성 트리를 TOON 압축 맵으로 반환합니다. 상호작용 지정을 위해 순차적인 `[ID: N]` 태그를 갖는 상호작용 요소(버튼, 입력 필드, 체크박스 등)의 고밀도 목록을 생성합니다.
- `nairobi_interact`: TOON 노드 ID를 사용하여 UI 요소에 대해 의미론적 동작(`click`, `activate`, `focus`)을 실행합니다.
- `nairobi_type_text`: TOON 노드 ID로 지정된 입력 가능한 필드(Entry, TextArea 등)에 아토믹하게 텍스트를 입력합니다.

## 사용 방법
Nairobi Connector를 사용하는 에이전트는 다음 기본 루프를 따라야 합니다:
1. `nairobi_find_window`를 사용하여 대상 창을 지정합니다.
2. `nairobi_get_ui_map`을 통해 현재 상태를 확인합니다.
3. 대상 상호작용 요소의 TOON `[ID: N]`을 읽어들입니다.
4. `nairobi_interact` 또는 `nairobi_type_text`를 통해 해당 요소에 대해 동작을 실행합니다.
5. 상호작용을 다시 하기 전에 최신 ID를 가져오기 위해 2단계부터 반복합니다.

## 지원
Nairobi OS가 유용하다고 생각되면 다음을 통해 저희 독립 시스템 연구를 지원해 주시기 바랍니다:

[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20Me%20A%20Coffee-PayPal-blue.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_donations&business=chege.finance@gmail.com&item_name=Support+Nairobi+OS+Development)

## 라이선스
이 프로젝트는 **Apache License 2.0** 라이선스에 따라 사용 허가됩니다.  
*(참고: TOON 포맷 및 브리지 구현의 일부는 TOON 원작자들에게 저작권 공헌이 귀속됩니다.)*
