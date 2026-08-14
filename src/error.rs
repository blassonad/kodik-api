//! Ошибки, возвращаемые библиотекой.

use std::time::Duration;

/// Результат операции библиотеки Kodik API.
pub type Result<T> = std::result::Result<T, Error>;

/// Ошибка, возникшая при подготовке или выполнении запроса к Kodik API.
///
/// Варианты с вложенной ошибкой используют `#[from]`, поэтому операторы `?` в
/// клиенте автоматически сохраняют исходную причину как `source()`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Параметры запроса не прошли локальную валидацию.
    #[error("invalid Kodik request: {0}")]
    Validation(String),

    /// URI не может быть представлен типом `hyper::Uri`.
    #[error("invalid request URI: {0}")]
    InvalidUri(#[from] hyper::http::uri::InvalidUri),

    /// Не удалось собрать HTTP-запрос.
    #[error("could not build HTTP request: {0}")]
    RequestBuild(#[from] hyper::http::Error),

    /// TLS-соединение, сеть или HTTP-протокол вернули ошибку.
    #[error("Kodik HTTP transport error: {0}")]
    Transport(#[from] hyper::Error),

    /// Полный HTTP-запрос (получение заголовков и тела) не завершился за отведённое время.
    #[error("Kodik HTTP request did not complete within {} ms", .0.as_millis())]
    Timeout(Duration),

    /// Ответ превысил настроенное ограничение, поэтому его чтение было остановлено.
    #[error("Kodik HTTP response exceeded the configured {limit}-byte body limit")]
    ResponseBodyTooLarge {
        /// Максимально допустимый размер тела в байтах.
        limit: usize,
    },

    /// Тело ответа нельзя разобрать как JSON.
    #[error("could not decode Kodik JSON response: {0}")]
    Json(#[from] simd_json::Error),

    /// Сервер вернул HTTP-статус вне успешного диапазона `2xx`.
    #[error("Kodik API returned HTTP {status}: {body}")]
    HttpStatus {
        /// Числовой HTTP-статус.
        status: u16,
        /// Тело ответа, декодированное с потерями UTF-8 только для диагностики.
        body: String,
    },
}
