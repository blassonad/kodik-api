//! Ошибки, возвращаемые библиотекой.

use std::fmt;
use std::time::Duration;

/// Результат операции библиотеки Kodik API.
pub type Result<T> = std::result::Result<T, Error>;

/// Ошибка, возникшая при подготовке или выполнении запроса к Kodik API.
#[derive(Debug)]
pub enum Error {
    /// Параметры запроса не прошли локальную валидацию.
    Validation(String),
    /// URI не может быть представлен типом `hyper::Uri`.
    InvalidUri(hyper::http::uri::InvalidUri),
    /// Не удалось собрать HTTP-запрос.
    RequestBuild(hyper::http::Error),
    /// TLS-соединение, сеть или HTTP-протокол вернули ошибку.
    Transport(hyper::Error),
    /// Полный HTTP-запрос (получение заголовков и тела) не завершился за отведённое время.
    Timeout(Duration),
    /// Ответ превысил настроенное ограничение, поэтому его чтение было остановлено.
    ResponseBodyTooLarge {
        /// Максимально допустимый размер тела в байтах.
        limit: usize,
    },
    /// Тело ответа нельзя разобрать как JSON.
    Json(simd_json::Error),
    /// Сервер вернул HTTP-статус вне успешного диапазона `2xx`.
    HttpStatus {
        /// Числовой HTTP-статус.
        status: u16,
        /// Тело ответа, декодированное с потерями UTF-8 только для диагностики.
        body: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message) => write!(f, "invalid Kodik request: {message}"),
            Self::InvalidUri(error) => write!(f, "invalid request URI: {error}"),
            Self::RequestBuild(error) => write!(f, "could not build HTTP request: {error}"),
            Self::Transport(error) => write!(f, "Kodik HTTP transport error: {error}"),
            Self::Timeout(timeout) => write!(
                f,
                "Kodik HTTP request did not complete within {} ms",
                timeout.as_millis()
            ),
            Self::ResponseBodyTooLarge { limit } => write!(
                f,
                "Kodik HTTP response exceeded the configured {limit}-byte body limit"
            ),
            Self::Json(error) => write!(f, "could not decode Kodik JSON response: {error}"),
            Self::HttpStatus { status, body } => {
                write!(f, "Kodik API returned HTTP {status}: {body}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidUri(error) => Some(error),
            Self::RequestBuild(error) => Some(error),
            Self::Transport(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Validation(_)
            | Self::Timeout(_)
            | Self::ResponseBodyTooLarge { .. }
            | Self::HttpStatus { .. } => None,
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(value: hyper::Error) -> Self {
        Self::Transport(value)
    }
}

impl From<simd_json::Error> for Error {
    fn from(value: simd_json::Error) -> Self {
        Self::Json(value)
    }
}
