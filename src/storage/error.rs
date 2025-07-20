#[derive(Debug)]
pub enum StorageError {
    LockError,
    NotFound,
}

impl std::error::Error for StorageError {}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            StorageError::LockError => write!(f, "Storage is locked"),
            StorageError::NotFound => write!(f, "Full url not found"),
        }
    }
}
