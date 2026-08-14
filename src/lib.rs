//! # `kodik-api-client`
//!
//! Асинхронная типизированная библиотека для публичного [Kodik API][kodik].
//! Транспорт построен на **Hyper + Rustls**, выполнение — на **Tokio**, JSON
//! разбирается **simd-json**, структуры реализуют **Serde**, а исходящие запросы
//! ограничиваются **governor**. Ключ API передаётся только при создании клиента;
//! библиотека не записывает и не логирует его.
//!
//! ## Быстрый старт
//!
//! ```no_run
//! use kodik_api::{ContentType, Filters, KodikClient, SearchQuery};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = KodikClient::new(std::env::var("KODIK_API_TOKEN")?)?;
//! let query = SearchQuery {
//!     title: Some("Avatar".into()),
//!     filters: Filters::default().with_types([ContentType::ForeignMovie]),
//!     ..Default::default()
//! };
//! let response = client.search(query).await?;
//! for material in response.results {
//!     println!("{} — {}", material.id, material.title);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Метод [`KodikClient::list`] возвращает одну страницу. Сохраните `next_page`,
//! извлеките из неё курсор [`KodikClient::next_cursor`] и передайте его в
//! [`ListQuery::next`] следующего запроса. Полное описание входных параметров,
//! методов и моделей находится в [`docs/API.md`](https://github.com/blassonad/kodik-api/blob/feat/rust-kodik-client/docs/API.md).
//!
//! [kodik]: https://kodik-api.com

#![forbid(unsafe_code)]

mod client;
mod error;
mod models;
mod query;

pub use client::{KodikClient, KodikClientBuilder, DEFAULT_BASE_URL};
pub use error::{Error, Result};
pub use models::{
    BlockedSeason, BlockedSeasons, CollectionResponse, Episode, EpisodeData, Episodes, LabelCount,
    ListResponse, Material, MaterialData, SearchResponse, Season, Seasons, Translation,
    TranslationCount, YearCount,
};
pub use query::{
    ContentType, CountriesQuery, Filters, GenreSource, GenresQuery, LabelSort, ListQuery, ListSort,
    QualitiesQuery, RequestMethod, RequiredField, SearchQuery, SortOrder, TranslationType,
    TranslationsQuery, YearSort, YearsQuery,
};
