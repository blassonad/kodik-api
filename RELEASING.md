# Release Guide

Этот документ описывает автоматический выпуск `kodik-api-client` в crates.io. Публикация crates.io **необратима**: опубликованную версию нельзя перезаписать или удалить. Поэтому merge изменения версии в `main` должен происходить только после review, зелёного CI и решения maintainer’а.[1]

## Предварительная настройка

Maintainer должен иметь подтверждённую учётную запись crates.io и права публикации для имени `kodik-api-client`. Для GitHub Actions создайте защищённый Environment `crates-io`, включите required reviewers и добавьте secret `CARGO_REGISTRY_TOKEN` с токеном scope `publish-new` или `publish-update`, ограниченным этим crate. Никогда не используйте обычный API-токен в коде, issue, PR или repository secret с более широкими правами, чем необходимо.

> Workflow **не использует** `CRATES_IO_PUBLISH_ENABLED` или аналогичный флаг. Каждый push в `main` читает версию из `Cargo.toml`, проверяет её существование в crates.io и публикует, если такой версии ещё нет. Если версия уже опубликована, workflow завершается без повторной отправки.

Рекомендуемая будущая альтернатива — [trusted publishing](https://crates.io/docs/trusted-publishing): после настройки OIDC в crates.io можно удалить статический `CARGO_REGISTRY_TOKEN`. До этого отсутствие секрета считается ошибкой публикации, а не причиной пропуска workflow.

## Подготовка версии

1. Создайте ветку от `main`.
2. Измените `version` в `Cargo.toml` по правилам SemVer.
3. Перенесите относящиеся к версии записи из `Unreleased` в `CHANGELOG.md`; добавьте дату и ссылку сравнения.
4. Проверьте документацию, примеры и public API на соответствие версии.
5. Запустите локальную проверку:

```bash
cargo fmt --check
cargo test --doc --all-features
cargo test --all-targets --all-features --locked
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo package --allow-dirty
cargo package --list
cargo publish --dry-run
```

6. Откройте и проверьте `target/package/kodik-api-client-X.Y.Z.crate`; архив не должен содержать секретов, `target/`, локальных файлов или ненужных больших assets.[1]
7. Откройте pull request, дождитесь обязательных проверок CI и выполните merge в `main`.

## Автоматическая публикация

Push в `main` запускает workflow `Publish to crates.io`. Job `prepare` получает версию пакета через `cargo metadata` и делает запрос к `https://crates.io/api/v1/crates/kodik-api-client/<version>`.

| Результат проверки | Действие workflow |
|---|---|
| Версия существует (`HTTP 200`) | Job `no-release-needed` объясняет, что публикация пропущена. |
| Версия отсутствует (`HTTP 404`) | Job `publish` ожидает approval Environment `crates-io`, выполняет format, docs, doctests, tests, clippy и dry-run, затем выполняет `cargo publish --locked --token "$CARGO_REGISTRY_TOKEN"`. |
| Иной HTTP-статус | Workflow завершается с ошибкой, чтобы не публиковать при неизвестном состоянии реестра. |

Таким образом, merge с новой версией является release-триггером. Сначала увеличивайте версию в отдельном PR, затем объединяйте его в `main`. После окончания workflow проверьте страницу crates.io и ссылку `documentation`; docs.rs строит документацию после поступления crate в реестр, используя `[package.metadata.docs.rs]`.[2]

## Откат

Если опубликованная версия содержит дефект, не пытайтесь перезаписать её. Сразу выпустите исправленную версию. Если критически важно предотвратить новые зависимости на дефектную версию, используйте `cargo yank --version X.Y.Z`; yank не удаляет код и не защищает уже скомпрометированные секреты. При утечке токена немедленно отзовите его в сервисе, где он был создан.[1]

## References

[1] [Cargo Book — Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)

[2] [docs.rs — Metadata](https://docs.rs/about/metadata)
