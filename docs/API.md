# Справочник API библиотеки `kodik-api`

Этот документ описывает **все публичные методы**, входные структуры, сериализацию параметров, возвращаемые модели и ошибки версии `0.1.0`. Библиотека представляет типизированную обёртку над публичными ресурсами Kodik: `/years`, `/genres`, `/countries`, `/qualities/v2`, `/translations/v2`, `/list` и `/search`.[1]

> Все запросы по умолчанию направляются на `https://kodik-api.com`. Токен является обязательным параметром исходного API; в этой библиотеке он хранится внутри `KodikClient`, а не повторяется в каждом запросе.[1]

| Принцип | Реализация |
|---|---|
| Runtime | Все сетевые методы — `async` и требуют Tokio runtime. |
| Транспорт | `hyper` использует Rustls через `hyper-rustls`; HTTP/1.1 и HTTP/2 включены. |
| JSON | Входящие ответы разбираются `simd_json::serde::from_slice`, а публичные структуры реализуют `serde::{Deserialize, Serialize}`. |
| Ограничение частоты | `governor` ожидает квоту до каждого HTTP-запроса. Клоны клиента используют одну квоту. |
| Секреты | Токен не имеет getter-метода и никогда не печатается библиотекой. |
| GET/POST | Методы без суффикса используют GET. Варианты `*_with_method` также принимают `RequestMethod::Post` и посылают form-encoded body. |

## 1. Создание и настройка клиента

```rust
use std::num::NonZeroU32;
use kodik_api::KodikClient;

let client = KodikClient::builder(std::env::var("KODIK_API_TOKEN")?)
    .requests_per_second(NonZeroU32::new(2).unwrap())
    .build()?;
```

`KodikClient::new(token)` создаёт эквивалентный клиент с базовым URL `https://kodik-api.com` и тремя запросами в секунду. `KodikClient::builder(token)` возвращает `KodikClientBuilder`, в котором можно изменить квоту и `base_url`. Последняя настройка рассчитана на тесты; в production следует оставлять официальный домен.[1]

| Тип или метод | Вход | Результат и правила |
|---|---|---|
| `KodikClient::new(token)` | `impl Into<String>` | `Result<KodikClient>`; пустой токен вызывает `Error::Validation`. |
| `KodikClient::builder(token)` | `impl Into<String>` | `KodikClientBuilder`. |
| `KodikClientBuilder::requests_per_second` | `NonZeroU32` | Задаёт общую квоту клиента. |
| `KodikClientBuilder::base_url` | `impl Into<String>` | Меняет origin; допускаются только URL с `http://` либо `https://`. |
| `KodikClientBuilder::build` | — | Создаёт TLS-коннектор с системными корневыми сертификатами. |
| `KodikClient::base_url` | — | Возвращает текущий origin; токен не раскрывается. |

Клиент реализует `Clone`, `Send` и `Sync` через используемые компоненты. Его безопасно передавать в `tokio::spawn`: соединения и rate limit будут совместными.

## 2. Методы ресурсов

Ниже приведены методы «по умолчанию» и их строгие типы. Все методы с `*_with_method` принимают дополнительно `RequestMethod::{Get, Post}` и возвращают тот же результат.

| Метод `KodikClient` | Endpoint | Вход | Выход |
|---|---|---|---|
| `years(YearsQuery)` | `/years` | `YearsQuery` | `CollectionResponse<YearCount>` |
| `genres(GenresQuery)` | `/genres` | `GenresQuery` | `CollectionResponse<LabelCount>` |
| `countries(CountriesQuery)` | `/countries` | `CountriesQuery` | `CollectionResponse<LabelCount>` |
| `qualities(QualitiesQuery)` | `/qualities/v2` | `QualitiesQuery` | `CollectionResponse<LabelCount>` |
| `translations(TranslationsQuery)` | `/translations/v2` | `TranslationsQuery` | `CollectionResponse<TranslationCount>` |
| `list(ListQuery)` | `/list` | `ListQuery` | `ListResponse` |
| `search(SearchQuery)` | `/search` | `SearchQuery` | `SearchResponse` |

### 2.1. Справочники

Справочники возвращают универсальную оболочку `CollectionResponse<T>`.

```rust
pub struct CollectionResponse<T> {
    pub time: String,
    pub total: u64,
    pub results: Vec<T>,
}
```

`time` — строка, сообщаемая сервером; `total` — число материалов, подходящих фильтрам; `results` — строки конкретного справочника.[1]

| Endpoint | Структура строки `results` | Поля |
|---|---|---|
| `/years` | `YearCount` | `year: i32`, `count: u64` |
| `/genres`, `/countries`, `/qualities/v2` | `LabelCount` | `title: String`, `count: u64` |
| `/translations/v2` | `TranslationCount` | `id: u64`, `title: String`, `count: u64` |

`YearsQuery` содержит `filters: Filters` и `sort: YearSort`, где `YearSort::{Year, Count}`. `GenresQuery` содержит `filters`, `genres_type: GenreSource` и `sort: LabelSort`. `GenreSource` имеет варианты `Kinopoisk`, `Shikimori`, `MyDramaList` и `All`; по умолчанию используется `Kinopoisk`. `CountriesQuery`, `QualitiesQuery` и `TranslationsQuery` содержат `filters` и `sort: LabelSort`, у которого варианты `Title` и `Count`.

```rust
use kodik_api::{ContentType, Filters, GenreSource, GenresQuery, LabelSort};

let query = GenresQuery {
    filters: Filters::default().with_types([ContentType::AnimeSerial]),
    genres_type: GenreSource::All,
    sort: LabelSort::Count,
};
```

### 2.2. Материалы `/list`

`ListQuery` получает страницу материалов. API использует курсорную пагинацию: после первой страницы приложение передаёт параметр `next`, полученный из `next_page` предыдущего ответа.[1]

| Поле `ListQuery` | Тип | Кодируемый параметр | Семантика |
|---|---|---|---|
| `filters` | `Filters` | См. раздел 3 | Общие ограничения выдачи. |
| `limit` | `Option<u8>` | `limit` | От 1 до 100; `None` оставляет дефолт Kodik. Ноль отвергается локально. |
| `sort` | `ListSort` | `sort` | `Year`, `CreatedAt`, `UpdatedAt`, `KinopoiskRating`, `ImdbRating`, `ShikimoriRating`. |
| `order` | `SortOrder` | `order` | `Asc` или `Desc`. |
| `with_seasons` | `bool` | `with_seasons=true` | Добавляет карту сезонов. |
| `with_episodes` | `bool` | `with_episodes=true` | Добавляет карты эпизодов, значениями становятся ссылки. |
| `with_episodes_data` | `bool` | `with_episodes_data=true` | Добавляет расширенные объекты эпизодов со ссылкой, названием и кадрами. |
| `with_page_links` | `bool` | `with_page_links=true` | Преобразует ссылки плеера в ссылки на страницы плеера. |
| `not_blocked_in` | `Vec<String>` | `not_blocked_in` | Коды стран без блокировки. |
| `not_blocked_for_me` | `bool` | `not_blocked_for_me=true` | Просит API применить геопроверку для страны запроса. |
| `with_material_data` | `bool` | `with_material_data=true` | Добавляет внешние сведения в `material_data`. |
| `next` | `Option<String>` | `next` | Курсор следующей страницы. |

`ListSort::default()` — `UpdatedAt`, `SortOrder::default()` — `Desc`, а булевы поля по умолчанию выключены. `ListQuery::validate()` выполняет локальную проверку до HTTP-вызова; `ListQuery::query_string()` полезен при отладке сериализации, но не добавляет токен.

#### Безопасный обход страниц

`next_page` исходного API является URL и, следовательно, может содержать токен. Не храните и не логируйте его в открытом виде. Вместо этого используйте `KodikClient::next_cursor(&url)`, который извлекает и percent-decode-ит только значение `next`.

```rust
use kodik_api::{KodikClient, ListQuery};

let first = client.list(ListQuery { limit: Some(100), ..Default::default() }).await?;
if let Some(next_url) = &first.next_page {
    let next = KodikClient::next_cursor(next_url)?;
    let second = client.list(ListQuery { next: Some(next), ..Default::default() }).await?;
    assert!(second.prev_page.is_some());
}
```

### 2.3. Поиск `/search`

`SearchQuery` объединяет критерии идентификации, правила соответствия, общие фильтры и настройки выдачи. В отличие от `/list`, у него **обязателен хотя бы один** критерий: `title`, `title_orig`, Kodik ID, `player_link`, любая коллекция внешних ID либо `worldart_link`.[1]

| Поле `SearchQuery` | Тип | Параметр API | Значение |
|---|---|---|---|
| `title` | `Option<String>` | `title` | Нечёткий поиск по основным и внешним названиям. |
| `title_orig` | `Option<String>` | `title_orig` | Нечёткий поиск только по оригинальному названию. |
| `strict` | `bool` | `strict=true` | Сохраняет порядок слов при поиске по названию. |
| `full_match` | `bool` | `full_match=true` | Требует полного регистронезависимого соответствия. |
| `id` | `Option<String>` | `id` | Kodik ID, например `movie-123`. |
| `player_link` | `Option<String>` | `player_link` | Любая ссылка на плеер. |
| `kinopoisk_ids` | `Vec<String>` | `kinopoisk_id` | Один или несколько ID КиноПоиска. |
| `imdb_ids` | `Vec<String>` | `imdb_id` | Один или несколько ID IMDb. |
| `mdl_ids` | `Vec<String>` | `mdl_id` | ID MyDramaList. |
| `worldart_animation_ids` | `Vec<String>` | `worldart_animation_id` | ID World Art в разделе аниме. |
| `worldart_cinema_ids` | `Vec<String>` | `worldart_cinema_id` | ID World Art в разделе кино. |
| `worldart_link` | `Option<String>` | `worldart_link` | Полная ссылка World Art. |
| `shikimori_ids` | `Vec<String>` | `shikimori_id` | ID Shikimori. |
| `limit` | `Option<u8>` | `limit` | От 1 до 100; 0 отвергается. |
| `filters` | `Filters` | См. раздел 3 | Общие условия. |
| `prioritize_translations` | `Vec<String>` | `prioritize_translations` | Приоритеты ID/`voice`/`subtitles` слева направо; `"0"` отключает дефолт API. |
| `unprioritize_translations` | `Vec<String>` | `unprioritize_translations` | Пониженные приоритеты ID/типов; `"0"` выключает серверный дефолт. |
| `prioritize_translation_type` | `Option<TranslationType>` | `prioritize_translation_type` | `Voice` или `Subtitles` раньше другого типа. |
| `with_seasons`, `season` | `bool`, `Option<u32>` | `with_seasons`, `season` | Включает сезоны; выбор сезона также вызывает добавление сезонов на стороне API. |
| `with_episodes`, `with_episodes_data` | `bool` | `with_episodes`, `with_episodes_data` | Включают краткие/расширенные серии. |
| `episode` | `Option<u32>` | `episode` | Требует заполненный `season`; библиотека валидирует зависимость. |
| `with_page_links` | `bool` | `with_page_links` | Ссылки на страницы плеера. |
| `not_blocked_in`, `not_blocked_for_me` | `Vec<String>`, `bool` | Одноимённые | Географические ограничения. |
| `with_material_data` | `bool` | `with_material_data` | Внешние метаданные. |

`SearchQuery::validate()` возвращает `Error::Validation`, если не указан критерий либо `episode` указан без `season`. Его можно вызвать явно, но сетевой метод вызывает валидацию автоматически.

```rust
use kodik_api::{KodikClient, SearchQuery, TranslationType};

let results = client.search(SearchQuery {
    kinopoisk_ids: vec!["251733".into()],
    with_material_data: true,
    with_episodes_data: true,
    season: Some(1),
    episode: Some(1),
    prioritize_translation_type: Some(TranslationType::Voice),
    ..Default::default()
}).await?;
```

## 3. `Filters`: общая схема фильтрации

`Filters` — единый набор параметров, разделяемый запросами. `Filters::default()` создаёт пустое условие. Методы `with_types`, `with_countries` и `with_genres` являются эргономичными помощниками; все поля также публичны для полной настройки.

| Группа | Поля | Формат |
|---|---|---|
| Основное | `types: Vec<ContentType>`, `year: Vec<i32>` | Запятая: `anime-serial,foreign-serial`; `year` поддерживается на `/list` и `/search`, а API справочников применяет подмножество фильтров. |
| Озвучки | `translation_ids`, `block_translations`, `translation_type` | Числовые ID или `TranslationType::{Voice,Subtitles}`. |
| Внешние поля | `has_field`, `has_field_and` | `RequiredField::{KinopoiskId, ImdbId, MdlId, WorldartLink, ShikimoriId}`. |
| Флаги | `camrip`, `lgbt` | `Some(true)`/`Some(false)` сериализуются явно; `None` не добавляет параметр. |
| Страны | `countries`, `countries_and` | Чувствительные к регистру названия стран. |
| Жанры | `genres`, `anime_genres`, `drama_genres`, `all_genres` и пары `_and` | Наборы жанров соответствующих источников. |
| Рейтинги/длительность | `duration`, четыре поля `*_rating`, `minimal_age` | Точная строка (`"7.0"`) либо диапазон (`"6.5-8.2"`). |
| Участники | `actors`, `directors`, `producers`, `writers`, `composers`, `editors`, `designers`, `operators` и пары `_and` | Списки персон. |
| Аниме/дорамы | `rating_mpaa`, `anime_kind`, `mydramalist_tags`, `anime_status`, `drama_status`, `all_status`, `anime_studios`, `anime_licensed_by` и нужные `_and` | Строки либо строки, объединяемые запятой. |

Перечисление `ContentType` не допускает опечаток и покрывает официальные значения: `ForeignMovie`, `SovietCartoon`, `ForeignCartoon`, `RussianCartoon`, `Anime`, `RussianMovie`, `CartoonSerial`, `DocumentarySerial`, `RussianSerial`, `ForeignSerial`, `AnimeSerial`, `MultiPartFilm`.[1]

```rust
use kodik_api::{ContentType, Filters, RequiredField, TranslationType};

let filters = Filters {
    translation_type: Some(TranslationType::Voice),
    has_field_and: vec![RequiredField::KinopoiskId, RequiredField::ImdbId],
    countries_and: vec!["США".into(), "Великобритания".into()],
    imdb_rating: Some("7.0-10".into()),
    genres: vec!["боевик".into(), "фантастика".into()],
    lgbt: Some(false),
    ..Filters::default()
}.with_types([ContentType::ForeignMovie]);
```

## 4. Модели результатов материалов

Оба ресурса `/list` и `/search` возвращают `Material`; различается только внешняя оболочка. `ListResponse` имеет `prev_page` и `next_page`, а `SearchResponse` — нет.

```rust
pub struct ListResponse {
    pub time: String,
    pub total: u64,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
    pub results: Vec<Material>,
}

pub struct SearchResponse {
    pub time: String,
    pub total: u64,
    pub results: Vec<Material>,
}
```

| Категория `Material` | Поля | Когда присутствуют |
|---|---|---|
| Идентичность | `id`, `material_type` (JSON `type`), `link`, `title`, `title_orig`, `other_title`, `year` | Основные поля; альтернативные названия и год могут отсутствовать. |
| Внешние ID | `kinopoisk_id`, `imdb_id`, `mdl_id`, `worldart_link`, `shikimori_id` | Только при доступности у Kodik. Хранятся строками, чтобы сохранить исходный формат идентификаторов. |
| Видео | `quality`, `camrip`, `lgbt`, `screenshots` | Необязательные скалярные поля, кадры по умолчанию — пустой вектор. |
| Озвучка | `translation: Translation` | Всегда состоит из `id`, `title`, `translation_type` (JSON `type`). |
| Временные/гео | `created_at`, `updated_at`, `blocked_countries` | Даты — строки ISO 8601; страны блокировки — вектор. |
| Сериал | `seasons`, `last_season`, `last_episode`, `episodes_count`, `blocked_seasons` | Только для сериалов и/или при параметрах `with_*`. |
| Внешние данные | `material_data: Option<MaterialData>` | Только при `with_material_data=true` и наличии данных источников. |

### 4.1. Сезоны, серии и блокировки

`Seasons` — это `BTreeMap<String, Season>`, где строковый ключ является номером сезона. Каждый `Season` содержит опциональные `link` и `episodes`. `Episodes` — `BTreeMap<String, Episode>`.

`Episode` — непомеченный enum, который точно отражает два режима API:

| Вариант | Параметр запроса | Данные |
|---|---|---|
| `Episode::Link(String)` | `with_episodes=true` | URL плеера серии. |
| `Episode::Data(EpisodeData)` | `with_episodes_data=true` | `link`, необязательный `title`, список `screenshots`. |

`BlockedSeasons` аналогично поддерживает две формы JSON: `BlockedSeasons::All(String)` для строки `"all"` и `BlockedSeasons::BySeason(BTreeMap<String, BlockedSeason>)` для карты. Значение карты — `BlockedSeason::All(String)` либо `BlockedSeason::Episodes(Vec<String>)`.

### 4.2. `MaterialData`

`MaterialData` имеет все поля `Option`, потому что Kodik не обещает сведения из КиноПоиска, Shikimori или MyDramaList для каждого материала.[1]

| Смысл | Поля `MaterialData` |
|---|---|
| Названия и описание | `title`, `anime_title`, `title_en`, `other_titles`, `other_titles_en`, `other_titles_jp`, `anime_license_name`, `tagline`, `description`, `anime_description` |
| Лицензирование и статусы | `anime_licensed_by`, `anime_kind`, `mydramalist_tags`, `all_status`, `anime_status`, `drama_status` |
| Изображения и базовые данные | `year`, `poster_url`, `anime_poster_url`, `drama_poster_url`, `screenshots`, `duration`, `countries` |
| Жанры и студия | `all_genres`, `genres`, `anime_genres`, `drama_genres`, `anime_studios` |
| Оценки | `kinopoisk_rating`, `kinopoisk_votes`, `imdb_rating`, `imdb_votes`, `shikimori_rating`, `shikimori_votes`, `mydramalist_rating`, `mydramalist_votes` |
| Даты и возраст | `premiere_ru`, `premiere_world`, `aired_at`, `released_at`, `next_episode_at`, `rating_mpaa`, `minimal_age` |
| Эпизоды и участники | `episodes_total`, `episodes_aired`, `actors`, `directors`, `producers`, `writers`, `composers`, `editors`, `designers`, `operators` |

## 5. Кодирование параметров

Библиотека не зависит от URL-кодировщика стороннего пакета. Строки UTF-8 percent-encode-ятся согласно безопасному набору unreserved (`A-Z`, `a-z`, `0-9`, `-`, `_`, `.`, `~`), поэтому кириллица, пробелы, запятые и URL в значениях корректно передаются серверу. `query_string()` у параметров возвращает только пользовательские параметры: токен добавляется позднее, внутри клиента.

```rust
use kodik_api::{ContentType, Filters, YearsQuery, YearSort};

let encoded = YearsQuery {
    filters: Filters::default()
        .with_types([ContentType::AnimeSerial])
        .with_countries(["США", "Россия"]),
    sort: YearSort::Count,
}.query_string();

assert!(encoded.contains("types=anime-serial"));
assert!(encoded.contains("countries=%D0%A1%D0%A8%D0%90%2C"));
```

## 6. Ошибки

Каждый асинхронный метод возвращает crate alias `kodik_api::Result<T>`, то есть `std::result::Result<T, kodik_api::Error>`.

| Вариант `Error` | Причина | Рекомендованная реакция |
|---|---|---|
| `Validation(String)` | Пустой токен, нулевой `limit`, пустой поиск, `episode` без `season`, некорректный базовый URL или курсор. | Исправить входной запрос; повторять бессмысленно. |
| `InvalidUri` | Собранный endpoint/URL нельзя разобрать в `hyper::Uri`. | Проверить test-only `base_url` и входные данные. |
| `RequestBuild` | Hyper отклонил составление запроса. | Считать ошибкой конфигурации. |
| `Transport(hyper::Error)` | Сеть, TLS или HTTP-протокол. | При необходимости повторить с backoff в приложении. |
| `Json(simd_json::Error)` | Успешный HTTP-ответ не соответствует ожидаемому JSON. | Зафиксировать тело/заголовки безопасно, проверить изменения API. |
| `HttpStatus { status, body }` | Код ответа не из диапазона 2xx. | Учитывать статус; не раскрывать `body`, если он содержит секреты. |

Либрари не выполняет автоматические повторы. Это осознанно: политика retry зависит от того, использует ли приложение GET/POST, какие лимиты установлены у конкретного ключа и какие требования предъявляются к задержке.

## 7. Проверка и примеры

В репозитории есть восемь модульных проверок: сериализация Unicode/списков, валидация `/search`, зависимость `episode → season`, извлечение курсора, GET/POST-запросы, десериализация `simd-json` и проверка обработки неуспешных HTTP-статусов. Запустите:

```bash
cargo fmt --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

`examples/search.rs` показывает безопасный поиск по названию, а `examples/list_all.rs` — обход всех страниц без выполнения URL из `next_page` напрямую.

## References

[1] [Kodik API — официальная документация](https://kodik-api.com/)
