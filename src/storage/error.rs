use std::{
    collections::HashMap,
    sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard},
};

#[derive(Debug, PartialEq)]
pub enum StorageError {
    LockError(String),
    NotFound,
}

impl std::error::Error for StorageError {}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StorageError::LockError(ref err) => write!(f, "{err}"),
            StorageError::NotFound => write!(f, "Full url not found"),
        }
    }
}

impl From<PoisonError<RwLockWriteGuard<'_, HashMap<String, String>>>> for StorageError {
    fn from(value: PoisonError<RwLockWriteGuard<'_, HashMap<String, String>>>) -> Self {
        Self::LockError(value.to_string())
    }
}

impl From<PoisonError<RwLockReadGuard<'_, HashMap<String, String>>>> for StorageError {
    fn from(value: PoisonError<RwLockReadGuard<'_, HashMap<String, String>>>) -> Self {
        Self::LockError(value.to_string())
    }
}
