# kodik-api-client

[![CI](https://github.com/blassonad/kodik-api/actions/workflows/ci.yml/badge.svg)](https://github.com/blassonad/kodik-api/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/kodik-api-client)](https://docs.rs/kodik-api-client)
[![crates.io](https://img.shields.io/crates/v/kodik-api-client)](https://crates.io/crates/kodik-api-client)
[![License](https://img.shields.io/crates/l/kodik-api-client)](LICENSE)

**`kodik-api-client`** — подробная асинхронная Rust-библиотека для публичного Kodik API. Она покрывает ресурсы `/years`, `/genres`, `/countries`, `/qualities/v2`, `/translations/v2`, `/list` и `/search`, предоставляет типизированные фильтры и модели ответов, а также поддерживает передачу параметров через `GET` и `POST`.[1]

> Библиотека **не содержит токенов**, не выводит их в лог и не включает реальных ключей в примеры. Используйте собственный токен только через переменную окружения `KODIK_API_TOKEN`.

| Компонент | Назначение |
|---|---|
| `hyper` + `hyper-rustls` | HTTP/1.1 и HTTP/2-клиент с TLS на Rustls; `hyper-rustls` является транспортным адаптером для Hyper и Rustls. |
| `tokio` | Асинхронный runtime для сетевых запросов и ожидания лимитера. |
| `simd-json` | Высокопроизводительное десериализующее чтение JSON-ответов. |
| `serde` | Строго типизированные модели запросов и ответов. |
| `governor` | Общий для клонов клиента rate limit; по умолчанию — 3 исходящих запроса в секунду. |

## Установка

После публикации добавьте dependency из crates.io; минимальная поддерживаемая версия Rust (MSRV) — **1.88**.

```toml
[dependencies]
kodik-api-client = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

До первой публикации используйте Git-ветку:

```toml
[dependencies]
kodik-api-client = { git = "https://github.com/blassonad/kodik-api", branch = "feat/rust-kodik-client" }
```

Локально проект проверяется командой:

```bash
cargo test --all-targets
```

## Быстрый старт

Выполните поиск фильма с нечётким названием. API-токен передаётся при создании [`KodikClient`](src/client.rs), а критерий поиска создаётся как [`SearchQuery`](src/query.rs). По документации Kodik хотя бы один критерий поиска обязателен; библиотека проверяет это до обращения к сети.[1]

```rust
use kodik_api::{ContentType, Filters, KodikClient, SearchQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KodikClient::new(std::env::var("KODIK_API_TOKEN")?)?;

    let query = SearchQuery {
        title: Some("Аватар смотреть онлайн".into()),
        filters: Filters::default().with_types([ContentType::ForeignMovie]),
        limit: Some(10),
        ..Default::default()
    };

    let response = client.search(query).await?;
    for item in response.results {
        println!("{} ({:?}) — {}", item.title, item.year, item.link);
    }
    Ok(())
}
```

## Методы клиента

| Метод | Kodik endpoint | Результат | Особенность |
|---|---|---|---|
| `years` | `/years` | `CollectionResponse<YearCount>` | Справочник годов; сортировка `YearSort`. |
| `genres` | `/genres` | `CollectionResponse<LabelCount>` | Поддерживает источник `GenreSource`. |
| `countries` | `/countries` | `CollectionResponse<LabelCount>` | Справочник стран с теми же общими фильтрами. |
| `qualities` | `/qualities/v2` | `CollectionResponse<LabelCount>` | Вторая версия справочника качеств. |
| `translations` | `/translations/v2` | `CollectionResponse<TranslationCount>` | Вторая версия справочника озвучек с ID. |
| `list` | `/list` | `ListResponse` | Постраничный обход всей базы по курсору `next`. |
| `search` | `/search` | `SearchResponse` | Нечёткий либо точный поиск названий и ID. |

Каждый метод также имеет пару `*_with_method(query, RequestMethod)`. По умолчанию используется `RequestMethod::Get`; `RequestMethod::Post` отправляет ту же кодированную форму в теле `application/x-www-form-urlencoded`, поскольку Kodik принимает параметры обоими способами.[1]

## Запросы и фильтры

Общие ограничения находятся в [`Filters`](src/query.rs). Массивы кодируются через запятую. Поля без суффикса `_and` означают «есть хотя бы одно значение», а соответствующие `_and` — «есть все значения», как определено в документации API.[1]

| Категория | Значимые поля `Filters` |
|---|---|
| Типы, годы и перевод | `types`, `year`, `translation_ids`, `block_translations`, `translation_type` |
| Идентификаторы и признаки | `has_field`, `has_field_and`, `camrip`, `lgbt` |
| Страны и жанры | `countries`, `countries_and`, `genres`, `anime_genres`, `drama_genres`, `all_genres` и варианты `_and` |
| Численные условия | `duration`, `kinopoisk_rating`, `imdb_rating`, `shikimori_rating`, `mydramalist_rating`, `minimal_age` |
| Персоны | `actors`, `directors`, `producers`, `writers`, `composers`, `editors`, `designers`, `operators` и варианты `_and` |
| Аниме и дорамы | `rating_mpaa`, `anime_kind`, `mydramalist_tags`, статусы, `anime_studios`, `anime_licensed_by` |

Диапазоны передаются строкой, например `duration: Some("40-80".into())` или `imdb_rating: Some("7.0-8.5".into())`. Это намеренно сохраняет синтаксис API без потери точности.

## Пагинация `/list`

`ListResponse` содержит `next_page`. Нельзя выполнять этот URL напрямую: он включает секретный токен в query string. Вместо этого извлеките только курсор через `KodikClient::next_cursor`, сохраните его в `ListQuery::next` и пошлите новый запрос библиотекой.

```rust
use kodik_api::{KodikClient, ListQuery};

async fn next_page(client: &KodikClient, first: &kodik_api::ListResponse)
    -> Result<Option<kodik_api::ListResponse>, kodik_api::Error>
{
    let Some(url) = &first.next_page else {
        return Ok(None);
    };

    let cursor = KodikClient::next_cursor(url)?;
    client.list(ListQuery { next: Some(cursor), ..Default::default() })
        .await
        .map(Some)
}
```

Полный перечень структур, всех полей и семантики доступен в [`docs/API.md`](docs/API.md). Рабочие, не содержащие ключа примеры находятся в [`examples/search.rs`](examples/search.rs) и [`examples/list_all.rs`](examples/list_all.rs). Процедура публикации в crates.io и создания GitHub Release описана в [`RELEASING.md`](RELEASING.md).

## Лицензия

Проект распространяется по лицензии [MIT](LICENSE).

## References

[1] [Kodik API — документация ресурсов](https://kodik-api.com/)
