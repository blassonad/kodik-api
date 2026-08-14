# Changelog

Все существенные изменения проекта документируются в этом файле. Формат основан на [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), а версии следуют [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Release-инфраструктура: CI, упаковка для crates.io, contribution- и security-политики, GitHub templates и автоматические проверки зависимостей.

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
