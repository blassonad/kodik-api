//! Асинхронный HTTP-клиент Kodik API на Hyper и Rustls.

use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use governor::{Quota, RateLimiter};
use hyper::body::HttpBody;
use hyper::client::HttpConnector;
use hyper::header::{ACCEPT, CONTENT_TYPE};
use hyper::{Body, Client, Method, Request, Uri};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use serde::de::DeserializeOwned;

use crate::error::{Error, Result};
use crate::models::{
    CollectionResponse, LabelCount, ListResponse, SearchResponse, TranslationCount, YearCount,
};
use crate::query::{
    CountriesQuery, GenresQuery, IntoParameters, ListQuery, Parameters, QualitiesQuery,
    RequestMethod, SearchQuery, TranslationsQuery, YearsQuery,
};

/// Адрес публичного Kodik API по умолчанию.
pub const DEFAULT_BASE_URL: &str = "https://kodik-api.com";

/// Максимальная продолжительность одного HTTP-обмена после прохождения rate limiter.
pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(20);
/// Максимальное число байтов, которые клиент прочитает из одного ответа.
pub const DEFAULT_MAX_RESPONSE_BODY_BYTES: usize = 8 * 1024 * 1024;

type HttpsClient = Client<HttpsConnector<HttpConnector>, Body>;
type DirectRateLimiter = governor::DefaultDirectRateLimiter;

/// Построитель [`KodikClient`].
///
/// По умолчанию клиент ограничивает себя тремя запросами в секунду. Ограничение
/// выполняется до каждой отправки запроса и совместно используется клонами клиента.
#[derive(Clone, Debug)]
pub struct KodikClientBuilder {
    token: String,
    base_url: String,
    requests_per_second: NonZeroU32,
    request_timeout: Duration,
    max_response_body_bytes: usize,
}

impl KodikClientBuilder {
    /// Создаёт построитель с API-токеном.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            base_url: DEFAULT_BASE_URL.into(),
            requests_per_second: NonZeroU32::new(3).expect("3 is non-zero"),
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            max_response_body_bytes: DEFAULT_MAX_RESPONSE_BODY_BYTES,
        }
    }

    /// Устанавливает предельное число исходящих запросов в секунду.
    pub fn requests_per_second(mut self, value: NonZeroU32) -> Self {
        self.requests_per_second = value;
        self
    }

    /// Устанавливает максимальную продолжительность HTTP-обмена после rate limiter.
    ///
    /// Значение не включает ожидание собственной квоты [`governor`], но включает
    /// получение заголовков и чтение всего ограниченного тела ответа.
    pub fn request_timeout(mut self, value: Duration) -> Self {
        self.request_timeout = value;
        self
    }

    /// Устанавливает максимальный размер тела одного ответа в байтах.
    ///
    /// Клиент останавливает чтение сразу после превышения этого значения и
    /// возвращает [`Error::ResponseBodyTooLarge`].
    pub fn max_response_body_bytes(mut self, value: usize) -> Self {
        self.max_response_body_bytes = value;
        self
    }

    /// Меняет базовый адрес. Предназначено прежде всего для интеграционных тестов.
    ///
    /// В обычном использовании сохраняйте значение по умолчанию
    /// `https://kodik-api.com`.
    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = value.into().trim_end_matches('/').into();
        self
    }

    /// Проверяет конфигурацию и создаёт клиент с системным хранилищем CA Rustls.
    pub fn build(self) -> Result<KodikClient> {
        if self.token.trim().is_empty() {
            return Err(Error::Validation("API token must not be empty".into()));
        }
        if !(self.base_url.starts_with("https://") || self.base_url.starts_with("http://")) {
            return Err(Error::Validation(
                "base URL must begin with http:// or https://".into(),
            ));
        }
        if self.request_timeout.is_zero() {
            return Err(Error::Validation(
                "request timeout must be greater than zero".into(),
            ));
        }
        if self.max_response_body_bytes == 0 {
            return Err(Error::Validation(
                "maximum response body size must be greater than zero".into(),
            ));
        }

        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        Ok(KodikClient {
            token: Arc::from(self.token),
            base_url: Arc::from(self.base_url),
            http: Client::builder().build(connector),
            limiter: Arc::new(RateLimiter::direct(Quota::per_second(
                self.requests_per_second,
            ))),
            request_timeout: self.request_timeout,
            max_response_body_bytes: self.max_response_body_bytes,
        })
    }
}

/// Многопоточно-безопасный клиент Kodik API.
///
/// Экземпляр можно свободно клонировать и передавать между задачами Tokio:
/// клоны разделяют пул HTTP-соединений и квоту [`governor`].
#[derive(Clone)]
pub struct KodikClient {
    token: Arc<str>,
    base_url: Arc<str>,
    http: HttpsClient,
    limiter: Arc<DirectRateLimiter>,
    request_timeout: Duration,
    max_response_body_bytes: usize,
}

impl KodikClient {
    /// Создаёт клиент с базовым адресом Kodik и лимитом в три запроса в секунду.
    pub fn new(token: impl Into<String>) -> Result<Self> {
        KodikClientBuilder::new(token).build()
    }

    /// Возвращает построитель с указанным API-токеном.
    pub fn builder(token: impl Into<String>) -> KodikClientBuilder {
        KodikClientBuilder::new(token)
    }

    /// Возвращает настроенный базовый URL, не раскрывая токен.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Получает доступные годы через `/years` методом `GET`.
    pub async fn years(&self, query: YearsQuery) -> Result<CollectionResponse<YearCount>> {
        self.years_with_method(query, RequestMethod::Get).await
    }

    /// Получает доступные годы, передавая параметры выбранным HTTP-методом.
    pub async fn years_with_method(
        &self,
        query: YearsQuery,
        method: RequestMethod,
    ) -> Result<CollectionResponse<YearCount>> {
        self.execute("/years", query, method).await
    }

    /// Получает жанры через `/genres` методом `GET`.
    pub async fn genres(&self, query: GenresQuery) -> Result<CollectionResponse<LabelCount>> {
        self.genres_with_method(query, RequestMethod::Get).await
    }

    /// Получает жанры, передавая параметры выбранным HTTP-методом.
    pub async fn genres_with_method(
        &self,
        query: GenresQuery,
        method: RequestMethod,
    ) -> Result<CollectionResponse<LabelCount>> {
        self.execute("/genres", query, method).await
    }

    /// Получает страны через `/countries` методом `GET`.
    pub async fn countries(&self, query: CountriesQuery) -> Result<CollectionResponse<LabelCount>> {
        self.countries_with_method(query, RequestMethod::Get).await
    }

    /// Получает страны, передавая параметры выбранным HTTP-методом.
    pub async fn countries_with_method(
        &self,
        query: CountriesQuery,
        method: RequestMethod,
    ) -> Result<CollectionResponse<LabelCount>> {
        self.execute("/countries", query, method).await
    }

    /// Получает качества второй версии через `/qualities/v2` методом `GET`.
    pub async fn qualities(&self, query: QualitiesQuery) -> Result<CollectionResponse<LabelCount>> {
        self.qualities_with_method(query, RequestMethod::Get).await
    }

    /// Получает качества, передавая параметры выбранным HTTP-методом.
    pub async fn qualities_with_method(
        &self,
        query: QualitiesQuery,
        method: RequestMethod,
    ) -> Result<CollectionResponse<LabelCount>> {
        self.execute("/qualities/v2", query, method).await
    }

    /// Получает озвучки второй версии через `/translations/v2` методом `GET`.
    pub async fn translations(
        &self,
        query: TranslationsQuery,
    ) -> Result<CollectionResponse<TranslationCount>> {
        self.translations_with_method(query, RequestMethod::Get)
            .await
    }

    /// Получает озвучки, передавая параметры выбранным HTTP-методом.
    pub async fn translations_with_method(
        &self,
        query: TranslationsQuery,
        method: RequestMethod,
    ) -> Result<CollectionResponse<TranslationCount>> {
        self.execute("/translations/v2", query, method).await
    }

    /// Получает одну страницу материалов через `/list` методом `GET`.
    pub async fn list(&self, query: ListQuery) -> Result<ListResponse> {
        self.list_with_method(query, RequestMethod::Get).await
    }

    /// Получает страницу `/list`, передавая параметры выбранным HTTP-методом.
    pub async fn list_with_method(
        &self,
        query: ListQuery,
        method: RequestMethod,
    ) -> Result<ListResponse> {
        self.execute("/list", query, method).await
    }

    /// Выполняет поиск через `/search` методом `GET`.
    pub async fn search(&self, query: SearchQuery) -> Result<SearchResponse> {
        self.search_with_method(query, RequestMethod::Get).await
    }

    /// Выполняет поиск, передавая параметры выбранным HTTP-методом.
    pub async fn search_with_method(
        &self,
        query: SearchQuery,
        method: RequestMethod,
    ) -> Result<SearchResponse> {
        self.execute("/search", query, method).await
    }

    /// Извлекает курсор `next` из URL `next_page`, возвращённого `/list`.
    ///
    /// Передайте результат в [`ListQuery::next`], а затем вызовите [`Self::list`].
    /// Метод намеренно не делает запрос к произвольному URL и не передаёт токен
    /// за пределы настроенного домена.
    pub fn next_cursor(next_page_url: &str) -> Result<String> {
        let query = next_page_url
            .split_once('?')
            .map(|(_, query)| query)
            .ok_or_else(|| Error::Validation("next_page URL has no query string".into()))?;
        let encoded = query
            .split('&')
            .find_map(|item| item.split_once('=').filter(|(key, _)| *key == "next"))
            .map(|(_, value)| value)
            .ok_or_else(|| Error::Validation("next_page URL has no next cursor".into()))?;
        percent_decode(encoded)
    }

    async fn execute<Q, T>(&self, endpoint: &str, query: Q, method: RequestMethod) -> Result<T>
    where
        Q: IntoParameters,
        T: DeserializeOwned,
    {
        let mut parameters = Parameters::default();
        parameters.push("token", self.token.as_ref());
        parameters.extend(query.into_parameters()?);
        let request = self.build_request(endpoint, parameters, method)?;

        self.limiter.until_ready().await;
        let timeout = self.request_timeout;
        let max_response_body_bytes = self.max_response_body_bytes;
        let (status, body) = tokio::time::timeout(timeout, async {
            let response = self.http.request(request).await?;
            let status = response.status();
            let body = read_response_body(response.into_body(), max_response_body_bytes).await?;
            Ok::<_, Error>((status, body))
        })
        .await
        .map_err(|_| Error::Timeout(timeout))??;

        if !status.is_success() {
            return Err(Error::HttpStatus {
                status: status.as_u16(),
                body: String::from_utf8_lossy(&body).into_owned(),
            });
        }

        decode_json(&body)
    }

    fn build_request(
        &self,
        endpoint: &str,
        parameters: Parameters,
        request_method: RequestMethod,
    ) -> Result<Request<Body>> {
        let encoded = parameters.encoded();
        let uri: Uri = match request_method {
            RequestMethod::Get => format!("{}{}?{}", self.base_url, endpoint, encoded)
                .parse()
                .map_err(Error::InvalidUri)?,
            RequestMethod::Post => format!("{}{}", self.base_url, endpoint)
                .parse()
                .map_err(Error::InvalidUri)?,
        };

        let mut builder = Request::builder()
            .method(match request_method {
                RequestMethod::Get => Method::GET,
                RequestMethod::Post => Method::POST,
            })
            .uri(uri)
            .header(ACCEPT, "application/json");
        let body = match request_method {
            RequestMethod::Get => Body::empty(),
            RequestMethod::Post => {
                builder = builder.header(CONTENT_TYPE, "application/x-www-form-urlencoded");
                Body::from(encoded)
            }
        };
        builder.body(body).map_err(Error::RequestBuild)
    }
}

async fn read_response_body(mut body: Body, limit: usize) -> Result<Vec<u8>> {
    if body
        .size_hint()
        .upper()
        .is_some_and(|length| length > limit as u64)
    {
        return Err(Error::ResponseBodyTooLarge { limit });
    }

    let mut bytes = Vec::new();
    while let Some(chunk) = body.data().await {
        let chunk = chunk?;
        let total = bytes
            .len()
            .checked_add(chunk.len())
            .ok_or(Error::ResponseBodyTooLarge { limit })?;
        if total > limit {
            return Err(Error::ResponseBodyTooLarge { limit });
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

fn decode_json<T: DeserializeOwned>(source: &[u8]) -> Result<T> {
    let mut buffer = source.to_vec();
    simd_json::serde::from_slice(&mut buffer).map_err(Error::Json)
}

fn percent_decode(value: &str) -> Result<String> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'%' if index + 2 < bytes.len() => {
                let high = from_hex(bytes[index + 1])?;
                let low = from_hex(bytes[index + 2])?;
                decoded.push((high << 4) | low);
                index += 3;
            }
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8(decoded)
        .map_err(|_| Error::Validation("next cursor is not valid UTF-8".into()))
}

fn from_hex(byte: u8) -> Result<u8> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(Error::Validation(
            "next cursor has malformed percent encoding".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::{ContentType, Filters, ListQuery, SearchQuery};
    use hyper::StatusCode;

    #[test]
    fn next_cursor_is_percent_decoded() {
        let cursor = KodikClient::next_cursor(
            "https://kodik-api.com/list?token=hidden&next=WyIxMjMiLCI0NTYiXQ%3D%3D",
        )
        .unwrap();
        assert_eq!(cursor, "WyIxMjMiLCI0NTYiXQ==");
    }

    #[test]
    fn get_request_has_token_and_encoded_filters() {
        let client = KodikClient::builder("test token").build().unwrap();
        let query = ListQuery {
            filters: Filters::default().with_types([ContentType::AnimeSerial]),
            limit: Some(20),
            ..Default::default()
        };
        let mut parameters = Parameters::default();
        parameters.push("token", "test token");
        parameters.extend(query.into_parameters().unwrap());
        let request = client
            .build_request("/list", parameters, RequestMethod::Get)
            .unwrap();
        assert_eq!(request.method(), Method::GET);
        assert!(request
            .uri()
            .query()
            .unwrap()
            .contains("token=test%20token"));
        assert!(request
            .uri()
            .query()
            .unwrap()
            .contains("types=anime-serial"));
    }

    #[test]
    fn post_request_sets_form_content_type() {
        let client = KodikClient::builder("token").build().unwrap();
        let mut parameters = Parameters::default();
        parameters.push("token", "token");
        parameters.extend(
            SearchQuery {
                title: Some("Avatar".into()),
                ..Default::default()
            }
            .into_parameters()
            .unwrap(),
        );
        let request = client
            .build_request("/search", parameters, RequestMethod::Post)
            .unwrap();
        assert_eq!(request.method(), Method::POST);
        assert_eq!(
            request.headers().get(CONTENT_TYPE).unwrap(),
            "application/x-www-form-urlencoded"
        );
    }

    #[test]
    fn response_is_decoded_by_simd_json() {
        let result: SearchResponse = decode_json(
            br#"{"time":"1ms","total":1,"results":[{"id":"movie-1","type":"foreign-movie","link":"https://example.test","title":"Title","translation":{"id":1,"title":"Voice","type":"voice"}}]}"#,
        )
        .unwrap();
        assert_eq!(result.results[0].id, "movie-1");
        assert_eq!(result.results[0].translation.translation_type, "voice");
    }

    #[test]
    fn rejected_status_is_not_treated_as_success() {
        assert!(!StatusCode::TOO_MANY_REQUESTS.is_success());
    }

    #[test]
    fn builder_rejects_zero_timeout_and_body_limit() {
        let timeout_error = match KodikClient::builder("token")
            .request_timeout(Duration::ZERO)
            .build()
        {
            Err(error) => error,
            Ok(_) => panic!("zero timeout must be rejected"),
        };
        assert!(matches!(timeout_error, Error::Validation(_)));

        let body_limit_error = match KodikClient::builder("token")
            .max_response_body_bytes(0)
            .build()
        {
            Err(error) => error,
            Ok(_) => panic!("zero response body limit must be rejected"),
        };
        assert!(matches!(body_limit_error, Error::Validation(_)));
    }

    #[tokio::test]
    async fn response_body_limit_rejects_known_oversized_body_before_buffering() {
        let error = read_response_body(Body::from("12345"), 4)
            .await
            .unwrap_err();
        assert!(matches!(error, Error::ResponseBodyTooLarge { limit: 4 }));
    }
}
