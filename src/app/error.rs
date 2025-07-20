use url::Url;

use crate::storage::error::StorageError;

#[derive(Debug, PartialEq)]
pub enum AppError {
    UrlNotFound,
    StorageInternalError(String),
    UrlParseError,
}

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AppError::UrlNotFound => write!(f, "Url not found"),
            AppError::StorageInternalError(ref err) => write!(f, "Storage internal error: {err}"),
            AppError::UrlParseError => write!(f, "Url parsing error"),
        }
    }
}

impl From<StorageError> for AppError {
    fn from(value: StorageError) -> Self {
        match value {
            StorageError::LockError(err) => AppError::StorageInternalError(err),
            StorageError::NotFound => AppError::UrlNotFound,
        }
    }
}

impl From<url::ParseError> for AppError {
    fn from(_value: url::ParseError) -> Self {
        Self::UrlParseError
    }
}
