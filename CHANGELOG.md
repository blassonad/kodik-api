# Changelog

Все существенные изменения проекта документируются в этом файле. Формат основан на [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), а версии следуют [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Release-инфраструктура: CI, упаковка для crates.io, contribution- и security-политики, GitHub templates и автоматические проверки зависимостей.
- Настраиваемые `request_timeout` и `max_response_body_bytes` с безопасными значениями по умолчанию.
- Декларативная реализация публичного `Error` через `thiserror` с сохранением транспортных и JSON-источников ошибок.
- Реальные Kodik API integration-тесты объединены с основным CI workflow вместо отдельного workflow.

### Changed

- Минимальная поддерживаемая версия Rust повышена до `1.97.1`; CI проверяет именно эту версию.

### Fixed

- `SearchQuery` теперь отвергает пустые и состоящие из пробелов критерии вместо передачи `Some("")` в API.
- `ListQuery::limit` и `SearchQuery::limit` проверяют весь диапазон `1..=100`, включая верхнюю границу.
- Чтение HTTP-ответа прерывается при превышении предела тела вместо безусловной загрузки данных в память.

## [0.1.0] - 2026-08-14

### Added

- Асинхронный `KodikClient` на Hyper + Rustls и Tokio.
- Rate limiting исходящих запросов через `governor`.
- Типизированные методы для `/years`, `/genres`, `/countries`, `/qualities/v2`, `/translations/v2`, `/list` и `/search`.
- Формирование GET- и POST-запросов, валидация входов, cursor-пагинация и безопасная обработка HTTP/JSON-ошибок.
- Serde-модели материалов, сезонов, эпизодов, блокировок и внешних метаданных.
- Документация API, безопасные примеры и модульные тесты.

[Unreleased]: https://github.com/blassonad/kodik-api/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/blassonad/kodik-api/releases/tag/v0.1.0
