//! Типизированные параметры всех ресурсов Kodik API.
//!
//! Все строки с несколькими значениями сериализуются через запятую, как требует API.

use std::fmt;

use crate::error::{Error, Result};

/// HTTP-способ передачи параметров к Kodik API.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RequestMethod {
    /// Параметры передаются в URL как query string. Это значение по умолчанию.
    #[default]
    Get,
    /// Параметры передаются в теле `application/x-www-form-urlencoded`.
    Post,
}

/// Тип материала, который Kodik принимает в параметре `types`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentType {
    ForeignMovie,
    SovietCartoon,
    ForeignCartoon,
    RussianCartoon,
    Anime,
    RussianMovie,
    CartoonSerial,
    DocumentarySerial,
    RussianSerial,
    ForeignSerial,
    AnimeSerial,
    MultiPartFilm,
}

impl ContentType {
    /// Строковое значение Kodik API.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ForeignMovie => "foreign-movie",
            Self::SovietCartoon => "soviet-cartoon",
            Self::ForeignCartoon => "foreign-cartoon",
            Self::RussianCartoon => "russian-cartoon",
            Self::Anime => "anime",
            Self::RussianMovie => "russian-movie",
            Self::CartoonSerial => "cartoon-serial",
            Self::DocumentarySerial => "documentary-serial",
            Self::RussianSerial => "russian-serial",
            Self::ForeignSerial => "foreign-serial",
            Self::AnimeSerial => "anime-serial",
            Self::MultiPartFilm => "multi-part-film",
        }
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Тип перевода в Kodik.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TranslationType {
    Voice,
    Subtitles,
}

impl TranslationType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Voice => "voice",
            Self::Subtitles => "subtitles",
        }
    }
}

impl fmt::Display for TranslationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Поле внешнего идентификатора, требуемое параметрами `has_field` и `has_field_and`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequiredField {
    KinopoiskId,
    ImdbId,
    MdlId,
    WorldartLink,
    ShikimoriId,
}

impl RequiredField {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KinopoiskId => "kinopoisk_id",
            Self::ImdbId => "imdb_id",
            Self::MdlId => "mdl_id",
            Self::WorldartLink => "worldart_link",
            Self::ShikimoriId => "shikimori_id",
        }
    }
}

impl fmt::Display for RequiredField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Типы списка жанров, который должен вернуть `/genres`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GenreSource {
    /// Жанры КиноПоиска — значение API по умолчанию.
    #[default]
    Kinopoisk,
    Shikimori,
    MyDramaList,
    All,
}

impl GenreSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Kinopoisk => "kinopoisk",
            Self::Shikimori => "shikimori",
            Self::MyDramaList => "mydramalist",
            Self::All => "all",
        }
    }
}

impl fmt::Display for GenreSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Направление сортировки `/list`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    /// Убывание — значение API по умолчанию.
    #[default]
    Desc,
}

impl SortOrder {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Поле сортировки `/list`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ListSort {
    Year,
    CreatedAt,
    /// Время обновления — значение API по умолчанию.
    #[default]
    UpdatedAt,
    KinopoiskRating,
    ImdbRating,
    ShikimoriRating,
}

impl ListSort {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Year => "year",
            Self::CreatedAt => "created_at",
            Self::UpdatedAt => "updated_at",
            Self::KinopoiskRating => "kinopoisk_rating",
            Self::ImdbRating => "imdb_rating",
            Self::ShikimoriRating => "shikimori_rating",
        }
    }
}

impl fmt::Display for ListSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Сортировка строкового справочника: стран, жанров, качеств и озвучек.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LabelSort {
    /// По названию — значение API по умолчанию.
    #[default]
    Title,
    Count,
}

impl LabelSort {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Count => "count",
        }
    }
}

impl fmt::Display for LabelSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Сортировка списка лет.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum YearSort {
    /// По году — значение API по умолчанию.
    #[default]
    Year,
    Count,
}

impl YearSort {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Year => "year",
            Self::Count => "count",
        }
    }
}

impl fmt::Display for YearSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Общие фильтры материалов, доступные в `/years`, `/genres`, `/countries`,
/// `/qualities/v2`, `/translations/v2`, `/list` и `/search`.
///
/// Поля с суффиксом `_and` требуют наличия **всех** перечисленных значений.
/// Их аналоги без суффикса используют семантику «хотя бы одно». Диапазоны
/// (`duration`, рейтинги и `minimal_age`) нужно задавать строкой: `"40-80"`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Filters {
    pub types: Vec<ContentType>,
    pub year: Vec<i32>,
    pub translation_ids: Vec<u64>,
    pub block_translations: Vec<u64>,
    pub translation_type: Option<TranslationType>,
    pub has_field: Vec<RequiredField>,
    pub has_field_and: Vec<RequiredField>,
    pub camrip: Option<bool>,
    pub lgbt: Option<bool>,
    pub countries: Vec<String>,
    pub countries_and: Vec<String>,
    pub genres: Vec<String>,
    pub genres_and: Vec<String>,
    pub anime_genres: Vec<String>,
    pub anime_genres_and: Vec<String>,
    pub drama_genres: Vec<String>,
    pub drama_genres_and: Vec<String>,
    pub all_genres: Vec<String>,
    pub all_genres_and: Vec<String>,
    pub duration: Option<String>,
    pub kinopoisk_rating: Option<String>,
    pub imdb_rating: Option<String>,
    pub shikimori_rating: Option<String>,
    pub mydramalist_rating: Option<String>,
    pub actors: Vec<String>,
    pub actors_and: Vec<String>,
    pub directors: Vec<String>,
    pub directors_and: Vec<String>,
    pub producers: Vec<String>,
    pub producers_and: Vec<String>,
    pub writers: Vec<String>,
    pub writers_and: Vec<String>,
    pub composers: Vec<String>,
    pub composers_and: Vec<String>,
    pub editors: Vec<String>,
    pub editors_and: Vec<String>,
    pub designers: Vec<String>,
    pub designers_and: Vec<String>,
    pub operators: Vec<String>,
    pub operators_and: Vec<String>,
    pub rating_mpaa: Vec<String>,
    pub minimal_age: Option<String>,
    pub anime_kind: Vec<String>,
    pub mydramalist_tags: Vec<String>,
    pub mydramalist_tags_and: Vec<String>,
    pub anime_status: Vec<String>,
    pub drama_status: Vec<String>,
    pub all_status: Vec<String>,
    pub anime_studios: Vec<String>,
    pub anime_studios_and: Vec<String>,
    pub anime_licensed_by: Vec<String>,
    pub anime_licensed_by_and: Vec<String>,
}

impl Filters {
    /// Добавляет фильтр по типам материалов.
    pub fn with_types(mut self, values: impl IntoIterator<Item = ContentType>) -> Self {
        self.types.extend(values);
        self
    }

    /// Добавляет фильтр по странам с семантикой «хотя бы одна».
    pub fn with_countries(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.countries.extend(values.into_iter().map(Into::into));
        self
    }

    /// Добавляет фильтр по жанрам КиноПоиска с семантикой «хотя бы один».
    pub fn with_genres(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.genres.extend(values.into_iter().map(Into::into));
        self
    }

    fn encode_into(&self, output: &mut Parameters) {
        output.push_joined("types", &self.types);
        output.push_joined("year", &self.year);
        output.push_joined("translation_id", &self.translation_ids);
        output.push_joined("block_translations", &self.block_translations);
        output.push_option("translation_type", self.translation_type);
        output.push_joined("has_field", &self.has_field);
        output.push_joined("has_field_and", &self.has_field_and);
        output.push_bool("camrip", self.camrip);
        output.push_bool("lgbt", self.lgbt);
        output.push_joined("countries", &self.countries);
        output.push_joined("countries_and", &self.countries_and);
        output.push_joined("genres", &self.genres);
        output.push_joined("genres_and", &self.genres_and);
        output.push_joined("anime_genres", &self.anime_genres);
        output.push_joined("anime_genres_and", &self.anime_genres_and);
        output.push_joined("drama_genres", &self.drama_genres);
        output.push_joined("drama_genres_and", &self.drama_genres_and);
        output.push_joined("all_genres", &self.all_genres);
        output.push_joined("all_genres_and", &self.all_genres_and);
        output.push_string("duration", self.duration.as_deref());
        output.push_string("kinopoisk_rating", self.kinopoisk_rating.as_deref());
        output.push_string("imdb_rating", self.imdb_rating.as_deref());
        output.push_string("shikimori_rating", self.shikimori_rating.as_deref());
        output.push_string("mydramalist_rating", self.mydramalist_rating.as_deref());
        output.push_joined("actors", &self.actors);
        output.push_joined("actors_and", &self.actors_and);
        output.push_joined("directors", &self.directors);
        output.push_joined("directors_and", &self.directors_and);
        output.push_joined("producers", &self.producers);
        output.push_joined("producers_and", &self.producers_and);
        output.push_joined("writers", &self.writers);
        output.push_joined("writers_and", &self.writers_and);
        output.push_joined("composers", &self.composers);
        output.push_joined("composers_and", &self.composers_and);
        output.push_joined("editors", &self.editors);
        output.push_joined("editors_and", &self.editors_and);
        output.push_joined("designers", &self.designers);
        output.push_joined("designers_and", &self.designers_and);
        output.push_joined("operators", &self.operators);
        output.push_joined("operators_and", &self.operators_and);
        output.push_joined("rating_mpaa", &self.rating_mpaa);
        output.push_string("minimal_age", self.minimal_age.as_deref());
        output.push_joined("anime_kind", &self.anime_kind);
        output.push_joined("mydramalist_tags", &self.mydramalist_tags);
        output.push_joined("mydramalist_tags_and", &self.mydramalist_tags_and);
        output.push_joined("anime_status", &self.anime_status);
        output.push_joined("drama_status", &self.drama_status);
        output.push_joined("all_status", &self.all_status);
        output.push_joined("anime_studios", &self.anime_studios);
        output.push_joined("anime_studios_and", &self.anime_studios_and);
        output.push_joined("anime_licensed_by", &self.anime_licensed_by);
        output.push_joined("anime_licensed_by_and", &self.anime_licensed_by_and);
    }
}

/// Запрос к `/years`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct YearsQuery {
    pub filters: Filters,
    pub sort: YearSort,
}

impl YearsQuery {
    pub fn query_string(&self) -> String {
        self.parameters().encoded()
    }

    fn parameters(&self) -> Parameters {
        let mut parameters = Parameters::default();
        self.filters.encode_into(&mut parameters);
        parameters.push("sort", self.sort);
        parameters
    }
}

/// Запрос к `/genres`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GenresQuery {
    pub filters: Filters,
    pub genres_type: GenreSource,
    pub sort: LabelSort,
}

impl GenresQuery {
    pub fn query_string(&self) -> String {
        self.parameters().encoded()
    }

    fn parameters(&self) -> Parameters {
        let mut parameters = Parameters::default();
        self.filters.encode_into(&mut parameters);
        parameters.push("genres_type", self.genres_type);
        parameters.push("sort", self.sort);
        parameters
    }
}

/// Запрос к `/countries`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CountriesQuery {
    pub filters: Filters,
    pub sort: LabelSort,
}

impl CountriesQuery {
    pub fn query_string(&self) -> String {
        self.parameters().encoded()
    }

    fn parameters(&self) -> Parameters {
        let mut parameters = Parameters::default();
        self.filters.encode_into(&mut parameters);
        parameters.push("sort", self.sort);
        parameters
    }
}

/// Запрос к `/qualities/v2`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QualitiesQuery {
    pub filters: Filters,
    pub sort: LabelSort,
}

impl QualitiesQuery {
    pub fn query_string(&self) -> String {
        self.parameters().encoded()
    }

    fn parameters(&self) -> Parameters {
        let mut parameters = Parameters::default();
        self.filters.encode_into(&mut parameters);
        parameters.push("sort", self.sort);
        parameters
    }
}

/// Запрос к `/translations/v2`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TranslationsQuery {
    pub filters: Filters,
    pub sort: LabelSort,
}

impl TranslationsQuery {
    pub fn query_string(&self) -> String {
        self.parameters().encoded()
    }

    fn parameters(&self) -> Parameters {
        let mut parameters = Parameters::default();
        self.filters.encode_into(&mut parameters);
        parameters.push("sort", self.sort);
        parameters
    }
}

/// Запрос к `/list`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListQuery {
    pub filters: Filters,
    /// Количество результатов на странице, от 1 до 100. `None` передаёт выбор серверу.
    pub limit: Option<u8>,
    pub sort: ListSort,
    pub order: SortOrder,
    pub with_seasons: bool,
    pub with_episodes: bool,
    pub with_episodes_data: bool,
    pub with_page_links: bool,
    pub not_blocked_in: Vec<String>,
    pub not_blocked_for_me: bool,
    pub with_material_data: bool,
    /// Курсор API из поля `next_page`; передавайте уже percent-decoded значение параметра `next`.
    pub next: Option<String>,
}

impl ListQuery {
    /// Проверяет границы `limit` до сетевого вызова.
    pub fn validate(&self) -> Result<()> {
        if matches!(self.limit, Some(0)) {
            return Err(Error::Validation("list limit must be in 1..=100".into()));
        }
        Ok(())
    }

    pub fn query_string(&self) -> Result<String> {
        Ok(self.parameters()?.encoded())
    }

    fn parameters(&self) -> Result<Parameters> {
        self.validate()?;
        let mut parameters = Parameters::default();
        self.filters.encode_into(&mut parameters);
        if let Some(limit) = self.limit {
            parameters.push("limit", limit);
        }
        parameters.push("sort", self.sort);
        parameters.push("order", self.order);
        parameters.push_bool_if_true("with_seasons", self.with_seasons);
        parameters.push_bool_if_true("with_episodes", self.with_episodes);
        parameters.push_bool_if_true("with_episodes_data", self.with_episodes_data);
        parameters.push_bool_if_true("with_page_links", self.with_page_links);
        parameters.push_joined("not_blocked_in", &self.not_blocked_in);
        parameters.push_bool_if_true("not_blocked_for_me", self.not_blocked_for_me);
        parameters.push_bool_if_true("with_material_data", self.with_material_data);
        parameters.push_string("next", self.next.as_deref());
        Ok(parameters)
    }
}

/// Параметры точного и нечёткого поиска `/search`.
///
/// Хотя бы одно поле-критерий должно быть заполнено: название, Kodik ID, ссылка
/// плеера или один из внешних идентификаторов.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchQuery {
    pub title: Option<String>,
    pub title_orig: Option<String>,
    pub strict: bool,
    pub full_match: bool,
    pub id: Option<String>,
    pub player_link: Option<String>,
    pub kinopoisk_ids: Vec<String>,
    pub imdb_ids: Vec<String>,
    pub mdl_ids: Vec<String>,
    pub worldart_animation_ids: Vec<String>,
    pub worldart_cinema_ids: Vec<String>,
    pub worldart_link: Option<String>,
    pub shikimori_ids: Vec<String>,
    /// Максимум результатов, от 1 до 100. `None` оставляет ограничение API.
    pub limit: Option<u8>,
    pub filters: Filters,
    /// Последовательность ID/типов перевода с более высоким приоритетом. Значение `"0"`
    /// в списке отключает серверную приоритизацию по умолчанию.
    pub prioritize_translations: Vec<String>,
    /// Последовательность ID/типов перевода с более низким приоритетом.
    pub unprioritize_translations: Vec<String>,
    pub prioritize_translation_type: Option<TranslationType>,
    pub with_seasons: bool,
    pub season: Option<u32>,
    pub with_episodes: bool,
    pub with_episodes_data: bool,
    pub episode: Option<u32>,
    pub with_page_links: bool,
    pub not_blocked_in: Vec<String>,
    pub not_blocked_for_me: bool,
    pub with_material_data: bool,
}

impl SearchQuery {
    /// Валидирует обязательный критерий поиска и зависимость `episode -> season`.
    pub fn validate(&self) -> Result<()> {
        if matches!(self.limit, Some(0)) {
            return Err(Error::Validation("search limit must be in 1..=100".into()));
        }
        if self.episode.is_some() && self.season.is_none() {
            return Err(Error::Validation(
                "search episode requires a corresponding season".into(),
            ));
        }
        if self.title.is_none()
            && self.title_orig.is_none()
            && self.id.is_none()
            && self.player_link.is_none()
            && self.kinopoisk_ids.is_empty()
            && self.imdb_ids.is_empty()
            && self.mdl_ids.is_empty()
            && self.worldart_animation_ids.is_empty()
            && self.worldart_cinema_ids.is_empty()
            && self.worldart_link.is_none()
            && self.shikimori_ids.is_empty()
        {
            return Err(Error::Validation(
                "search requires title, title_orig, Kodik ID, player link, or an external ID"
                    .into(),
            ));
        }
        Ok(())
    }

    pub fn query_string(&self) -> Result<String> {
        Ok(self.parameters()?.encoded())
    }

    fn parameters(&self) -> Result<Parameters> {
        self.validate()?;
        let mut parameters = Parameters::default();
        parameters.push_string("title", self.title.as_deref());
        parameters.push_string("title_orig", self.title_orig.as_deref());
        parameters.push_bool_if_true("strict", self.strict);
        parameters.push_bool_if_true("full_match", self.full_match);
        parameters.push_string("id", self.id.as_deref());
        parameters.push_string("player_link", self.player_link.as_deref());
        parameters.push_joined("kinopoisk_id", &self.kinopoisk_ids);
        parameters.push_joined("imdb_id", &self.imdb_ids);
        parameters.push_joined("mdl_id", &self.mdl_ids);
        parameters.push_joined("worldart_animation_id", &self.worldart_animation_ids);
        parameters.push_joined("worldart_cinema_id", &self.worldart_cinema_ids);
        parameters.push_string("worldart_link", self.worldart_link.as_deref());
        parameters.push_joined("shikimori_id", &self.shikimori_ids);
        if let Some(limit) = self.limit {
            parameters.push("limit", limit);
        }
        self.filters.encode_into(&mut parameters);
        parameters.push_joined("prioritize_translations", &self.prioritize_translations);
        parameters.push_joined("unprioritize_translations", &self.unprioritize_translations);
        parameters.push_option(
            "prioritize_translation_type",
            self.prioritize_translation_type,
        );
        parameters.push_bool_if_true("with_seasons", self.with_seasons);
        if let Some(season) = self.season {
            parameters.push("season", season);
        }
        parameters.push_bool_if_true("with_episodes", self.with_episodes);
        parameters.push_bool_if_true("with_episodes_data", self.with_episodes_data);
        if let Some(episode) = self.episode {
            parameters.push("episode", episode);
        }
        parameters.push_bool_if_true("with_page_links", self.with_page_links);
        parameters.push_joined("not_blocked_in", &self.not_blocked_in);
        parameters.push_bool_if_true("not_blocked_for_me", self.not_blocked_for_me);
        parameters.push_bool_if_true("with_material_data", self.with_material_data);
        Ok(parameters)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Parameters(Vec<(String, String)>);

impl Parameters {
    pub(crate) fn push(&mut self, key: impl Into<String>, value: impl fmt::Display) {
        self.0.push((key.into(), value.to_string()));
    }

    fn push_string(&mut self, key: impl Into<String>, value: Option<&str>) {
        if let Some(value) = value.filter(|value| !value.is_empty()) {
            self.0.push((key.into(), value.to_owned()));
        }
    }

    fn push_option<T: fmt::Display>(&mut self, key: impl Into<String>, value: Option<T>) {
        if let Some(value) = value {
            self.push(key, value);
        }
    }

    fn push_bool(&mut self, key: impl Into<String>, value: Option<bool>) {
        if let Some(value) = value {
            self.push(key, value);
        }
    }

    fn push_bool_if_true(&mut self, key: impl Into<String>, value: bool) {
        if value {
            self.push(key, "true");
        }
    }

    fn push_joined<T: fmt::Display>(&mut self, key: impl Into<String>, values: &[T]) {
        if !values.is_empty() {
            let value = values
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",");
            self.0.push((key.into(), value));
        }
    }

    pub(crate) fn extend(&mut self, other: Parameters) {
        self.0.extend(other.0);
    }

    pub(crate) fn encoded(&self) -> String {
        self.0
            .iter()
            .map(|(key, value)| format!("{}={}", percent_encode(key), percent_encode(value)))
            .collect::<Vec<_>>()
            .join("&")
    }
}

pub(crate) trait IntoParameters {
    fn into_parameters(self) -> Result<Parameters>;
}

impl IntoParameters for YearsQuery {
    fn into_parameters(self) -> Result<Parameters> {
        Ok(self.parameters())
    }
}

impl IntoParameters for GenresQuery {
    fn into_parameters(self) -> Result<Parameters> {
        Ok(self.parameters())
    }
}

impl IntoParameters for CountriesQuery {
    fn into_parameters(self) -> Result<Parameters> {
        Ok(self.parameters())
    }
}

impl IntoParameters for QualitiesQuery {
    fn into_parameters(self) -> Result<Parameters> {
        Ok(self.parameters())
    }
}

impl IntoParameters for TranslationsQuery {
    fn into_parameters(self) -> Result<Parameters> {
        Ok(self.parameters())
    }
}

impl IntoParameters for ListQuery {
    fn into_parameters(self) -> Result<Parameters> {
        self.parameters()
    }
}

impl IntoParameters for SearchQuery {
    fn into_parameters(self) -> Result<Parameters> {
        self.parameters()
    }
}

fn percent_encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push(hex(byte >> 4));
            encoded.push(hex(byte & 0x0f));
        }
    }
    encoded
}

const fn hex(nibble: u8) -> char {
    match nibble {
        0..=9 => (b'0' + nibble) as char,
        _ => (b'A' + nibble - 10) as char,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_unicode_and_comma_separated_values() {
        let query = YearsQuery {
            filters: Filters::default()
                .with_types([ContentType::AnimeSerial])
                .with_countries(["США", "Россия"]),
            sort: YearSort::Count,
        };
        assert_eq!(
            query.query_string(),
            "types=anime-serial&countries=%D0%A1%D0%A8%D0%90%2C%D0%A0%D0%BE%D1%81%D1%81%D0%B8%D1%8F&sort=count"
        );
    }

    #[test]
    fn search_requires_a_criterion() {
        let error = SearchQuery::default().validate().unwrap_err();
        assert!(error.to_string().contains("search requires"));
    }

    #[test]
    fn episode_requires_season() {
        let query = SearchQuery {
            title: Some("Test".into()),
            episode: Some(4),
            ..Default::default()
        };
        assert!(query.validate().is_err());
    }
}
