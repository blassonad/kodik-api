# Исследование требований к выпуску

Проверено 14 августа 2026 года.

| Источник | Практический вывод |
|---|---|
| Cargo Book: Publishing | Перед публикацией следует выполнить `cargo publish --dry-run` (или `cargo package`), проверить состав архива через `cargo package --list`, заполнить metadata манифеста и подготовить changelog и Git tag. Публикации crates.io неизменяемы. |
| docs.rs metadata | Секция `[package.metadata.docs.rs]` задаёт параметры сборки API-документации, включая `all-features`, `rustdoc-args`, default target и дополнительные targets. |
| GitHub Actions: Rust | CI может использовать `actions/checkout`, cache Cargo и стандартные `cargo build`/`cargo test`; publishing требует отдельного секрета либо доверенной публикации. |

Решения для этого репозитория:

1. Добавить метаданные crates.io: `homepage`, `documentation`, расширенные ключевые слова и категории, `include`, `publish = ["crates-io"]`.
2. Оставить публикацию ручным безопасным workflow по tag `v*`, но без встроенного токена. Workflow использует `CARGO_REGISTRY_TOKEN` только при наличии и требует Environment `crates-io`.
3. Добавить CI для MSRV, stable и beta; форматирование, clippy с `-D warnings`, тесты, doctests, docs и `cargo package --allow-dirty`.
4. Добавить `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, GitHub issue/PR templates, Dependabot и label/repository topics.

## Источники

[1] [Cargo Book — Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)

[2] [docs.rs — Metadata](https://docs.rs/about/metadata)

[3] [GitHub Docs — Building and testing Rust](https://docs.github.com/actions/tutorials/build-and-test-code/building-and-testing-rust)
