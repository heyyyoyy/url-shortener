use crate::storage::error::StorageError;

#[derive(Debug, PartialEq)]
pub enum AppError {
    UrlNotFound,
    StorageInternalError,
}

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AppError::UrlNotFound => write!(f, "Url not found"),
            AppError::StorageInternalError => write!(f, "Storage internal error"),
        }
    }
}

impl From<StorageError> for AppError {
    fn from(value: StorageError) -> Self {
        match value {
            StorageError::LockError => AppError::StorageInternalError,
            StorageError::NotFound => AppError::UrlNotFound,
        }
    }
}
