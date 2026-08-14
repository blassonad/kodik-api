# Release Guide

Этот документ описывает выпуск `kodik-api-client` в crates.io и соответствующий GitHub Release. Публикация crates.io **необратима**: опубликованную версию нельзя перезаписать или удалить. Поэтому выполняйте процедуру только после review, зелёного CI и явного решения maintainer’а.[1]

## Предварительная настройка

Maintainer должен иметь подтверждённую учётную запись crates.io и права публикации для имени `kodik-api-client`. Для GitHub Actions создайте защищённый Environment `crates-io`, включите required reviewers и добавьте secret `CARGO_REGISTRY_TOKEN` с токеном scope `publish-new` или `publish-update`, ограниченным этим crate. Никогда не используйте обычный API-токен в коде, issue, PR или repository secret с более широкими правами, чем необходимо.

Рекомендуемая будущая альтернатива — [trusted publishing](https://crates.io/docs/trusted-publishing): после первой ручной публикации настройте OIDC для GitHub Actions в crates.io и удалите статический токен. До настройки trusted publishing release-workflow безопасно не публикует crate, если `CARGO_REGISTRY_TOKEN` отсутствует.

## Подготовка версии

1. Создайте ветку `release/vX.Y.Z` от актуальной основной ветки.
2. Измените `version` в `Cargo.toml` по правилам SemVer.
3. Перенесите относящиеся к версии записи из `Unreleased` в `CHANGELOG.md`; добавьте дату и ссылку сравнения.
4. Проверьте документацию, примеры и public API на соответствие версии.
5. Запустите локальную проверку:

```bash
cargo fmt --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items
cargo package --allow-dirty
cargo package --list
cargo publish --dry-run
```

6. Откройте и проверьте `target/package/kodik-api-client-X.Y.Z.crate`; архив не должен содержать секретов, `target/`, локальных файлов или ненужных больших assets.[1]
7. Откройте pull request, дождитесь обязательных проверок CI и выполните merge.

## Публикация

1. На merged commit создайте аннотированный тег `vX.Y.Z` и отправьте его:

```bash
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin vX.Y.Z
```

2. Workflow `Release` выполняет повторную проверку crate, запрашивает approval Environment `crates-io`, публикует с `cargo publish --locked` и создаёт GitHub Release с автоматически сгенерированными release notes.
3. После окончания workflow проверьте страницу crates.io и ссылку `documentation`. docs.rs строит документацию после поступления crate в реестр; параметры сборки берутся из `[package.metadata.docs.rs]`.[2]
4. Проверьте release artifact и обновите `Unreleased`-ссылку для следующей версии.

## Откат

Если опубликованная версия содержит дефект, не пытайтесь перезаписать её. Сразу выпустите исправленную версию. Если критически важно предотвратить новые зависимости на дефектную версию, используйте `cargo yank --version X.Y.Z`; yank не удаляет код и не защищает уже скомпрометированные секреты. При утечке токена немедленно отзовите его в сервисе, где он был создан.[1]

## References

[1] [Cargo Book — Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)

[2] [docs.rs — Metadata](https://docs.rs/about/metadata)
