//! Типы JSON-ответов Kodik API.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Универсальная оболочка ответов справочников: годы, жанры, страны, качества и озвучки.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CollectionResponse<T> {
    /// Время обработки запроса, сообщаемое API.
    pub time: String,
    /// Общее число материалов, удовлетворяющих фильтрам.
    pub total: u64,
    /// Строки запрошенного справочника.
    pub results: Vec<T>,
}

/// Строка ресурса `/years`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct YearCount {
    /// Год выпуска материала.
    pub year: i32,
    /// Число материалов данного года.
    pub count: u64,
}

/// Строка ресурсов `/genres`, `/countries` и `/qualities/v2`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LabelCount {
    /// Текстовое значение справочника.
    pub title: String,
    /// Число материалов с этим значением.
    pub count: u64,
}

/// Строка ресурса `/translations/v2`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct TranslationCount {
    /// Внутренний ID озвучки Kodik.
    pub id: u64,
    /// Название озвучки.
    pub title: String,
    /// Число материалов с данной озвучкой.
    pub count: u64,
}

/// Страница результатов `/list`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ListResponse {
    /// Время обработки запроса, сообщаемое API.
    pub time: String,
    /// Общее число материалов, удовлетворяющих фильтрам.
    pub total: u64,
    /// URL предыдущей страницы или `null` на первой странице.
    #[serde(default)]
    pub prev_page: Option<String>,
    /// URL следующей страницы или `null` на последней странице.
    #[serde(default)]
    pub next_page: Option<String>,
    /// Материалы текущей страницы.
    pub results: Vec<Material>,
}

/// Ответ ресурса `/search`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SearchResponse {
    /// Время обработки запроса, сообщаемое API.
    pub time: String,
    /// Количество найденных материалов.
    pub total: u64,
    /// Материалы, отсортированные API по релевантности и приоритету озвучек.
    pub results: Vec<Material>,
}

/// Материал: фильм, сериал, мультфильм, аниме или иной результат Kodik.
///
/// Поля, которые Kodik возвращает не для каждого материала или лишь при включённых
/// параметрах `with_*`, представлены как `Option`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Material {
    /// Уникальный идентификатор Kodik, например `movie-452654`.
    pub id: String,
    /// Тип материала, например `foreign-movie` или `anime-serial`.
    #[serde(rename = "type")]
    pub material_type: String,
    /// Ссылка на плеер или, при `with_page_links=true`, на страницу плеера.
    pub link: String,
    /// Локализованное название.
    pub title: String,
    /// Оригинальное название, если есть.
    #[serde(default)]
    pub title_orig: Option<String>,
    /// Дополнительное название, часто используемое для аниме.
    #[serde(default)]
    pub other_title: Option<String>,
    /// Год выпуска.
    #[serde(default)]
    pub year: Option<i32>,
    /// ID КиноПоиска.
    #[serde(default)]
    pub kinopoisk_id: Option<String>,
    /// ID IMDb, обычно вида `tt...`.
    #[serde(default)]
    pub imdb_id: Option<String>,
    /// ID MyDramaList.
    #[serde(default)]
    pub mdl_id: Option<String>,
    /// Полная ссылка на World Art.
    #[serde(default)]
    pub worldart_link: Option<String>,
    /// Числовой ID Shikimori. Представлен строкой, чтобы не терять формат API.
    #[serde(default)]
    pub shikimori_id: Option<String>,
    /// Качество видео.
    #[serde(default)]
    pub quality: Option<String>,
    /// Признак записи с экрана.
    #[serde(default)]
    pub camrip: Option<bool>,
    /// Признак наличия LGBT-сцен.
    #[serde(default)]
    pub lgbt: Option<bool>,
    /// Озвучка либо субтитры, привязанные к этому материалу.
    pub translation: Translation,
    /// Дата добавления в ISO 8601, если отдана API.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Дата последнего обновления в ISO 8601, если отдана API.
    #[serde(default)]
    pub updated_at: Option<String>,
    /// Коды стран, в которых просмотр ограничен.
    #[serde(default)]
    pub blocked_countries: Vec<String>,
    /// Сезоны. API добавляет поле при `with_seasons`, `with_episodes` или `with_episodes_data`.
    #[serde(default)]
    pub seasons: Option<Seasons>,
    /// Номер последнего сезона сериала.
    #[serde(default)]
    pub last_season: Option<u32>,
    /// Номер последней серии сериала.
    #[serde(default)]
    pub last_episode: Option<u32>,
    /// Общее число эпизодов сериала.
    #[serde(default)]
    pub episodes_count: Option<u32>,
    /// Ограничения на сезоны и эпизоды сериала.
    #[serde(default)]
    pub blocked_seasons: Option<BlockedSeasons>,
    /// Ссылки на кадры. Для сериалов без детальных эпизодов это кадры первой серии.
    #[serde(default)]
    pub screenshots: Vec<String>,
    /// Внешние метаданные КиноПоиска, Shikimori и MyDramaList при `with_material_data=true`.
    #[serde(default)]
    pub material_data: Option<MaterialData>,
}

/// Привязанная к материалу озвучка или субтитры.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Translation {
    /// Внутренний ID озвучки Kodik.
    pub id: u64,
    /// Отображаемое название озвучки.
    pub title: String,
    /// Тип перевода: обычно `voice` или `subtitles`.
    #[serde(rename = "type")]
    pub translation_type: String,
}

/// Коллекция сезонов, ключами которой являются номера сезонов как строки JSON.
pub type Seasons = BTreeMap<String, Season>;

/// Сезон сериала.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Season {
    /// Ссылка на плеер сезона.
    #[serde(default)]
    pub link: Option<String>,
    /// Эпизоды, доступные при `with_episodes` или `with_episodes_data`.
    #[serde(default)]
    pub episodes: Option<Episodes>,
}

/// Коллекция серий, ключами которой являются номера эпизодов как строки JSON.
pub type Episodes = BTreeMap<String, Episode>;

/// Эпизод в кратком или расширенном виде.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Episode {
    /// Сокращённый вид `with_episodes=true`: непосредственно URL плеера серии.
    Link(String),
    /// Расширенный вид `with_episodes_data=true`.
    Data(EpisodeData),
}

/// Данные одной серии при `with_episodes_data=true`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct EpisodeData {
    /// Ссылка на плеер эпизода.
    pub link: String,
    /// Название серии, если API располагает им.
    #[serde(default)]
    pub title: Option<String>,
    /// Кадры из этой серии.
    #[serde(default)]
    pub screenshots: Vec<String>,
}

/// Описание блокировок сериалов. Kodik может вернуть строку `"all"` либо карту сезонов.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum BlockedSeasons {
    /// Все сезоны сериала полностью заблокированы.
    All(String),
    /// Блокировки отдельных сезонов.
    BySeason(BTreeMap<String, BlockedSeason>),
}

/// Блокировка одного сезона.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum BlockedSeason {
    /// Все эпизоды сезона заблокированы; ожидаемое значение — `"all"`.
    All(String),
    /// Заблокированы только эпизоды с указанными номерами.
    Episodes(Vec<String>),
}

/// Дополнительные сведения о материале из КиноПоиска, Shikimori и MyDramaList.
///
/// Каждый член необязателен: Kodik не возвращает данные, которых нет в источниках.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct MaterialData {
    pub title: Option<String>,
    pub anime_title: Option<String>,
    pub title_en: Option<String>,
    pub other_titles: Option<Vec<String>>,
    pub other_titles_en: Option<Vec<String>>,
    pub other_titles_jp: Option<Vec<String>>,
    pub anime_license_name: Option<String>,
    pub anime_licensed_by: Option<Vec<String>>,
    pub anime_kind: Option<String>,
    pub mydramalist_tags: Option<Vec<String>>,
    pub all_status: Option<String>,
    pub anime_status: Option<String>,
    pub drama_status: Option<String>,
    pub year: Option<i32>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub anime_description: Option<String>,
    pub poster_url: Option<String>,
    pub anime_poster_url: Option<String>,
    pub drama_poster_url: Option<String>,
    pub screenshots: Option<Vec<String>>,
    pub duration: Option<u32>,
    pub countries: Option<Vec<String>>,
    pub all_genres: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
    pub anime_genres: Option<Vec<String>>,
    pub drama_genres: Option<Vec<String>>,
    pub anime_studios: Option<Vec<String>>,
    pub kinopoisk_rating: Option<f64>,
    pub kinopoisk_votes: Option<u64>,
    pub imdb_rating: Option<f64>,
    pub imdb_votes: Option<u64>,
    pub shikimori_rating: Option<f64>,
    pub shikimori_votes: Option<u64>,
    pub mydramalist_rating: Option<f64>,
    pub mydramalist_votes: Option<u64>,
    pub premiere_ru: Option<String>,
    pub premiere_world: Option<String>,
    pub aired_at: Option<String>,
    pub released_at: Option<String>,
    pub next_episode_at: Option<String>,
    pub rating_mpaa: Option<String>,
    pub minimal_age: Option<u32>,
    pub episodes_total: Option<u32>,
    pub episodes_aired: Option<u32>,
    pub actors: Option<Vec<String>>,
    pub directors: Option<Vec<String>>,
    pub producers: Option<Vec<String>>,
    pub writers: Option<Vec<String>>,
    pub composers: Option<Vec<String>>,
    pub editors: Option<Vec<String>>,
    pub designers: Option<Vec<String>>,
    pub operators: Option<Vec<String>>,
}
