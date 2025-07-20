use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;

use crate::{
    app::{
        command::generate_url::GenerateShortUrlRepository,
        query::get_full_url::GetFullUrlRepository,
    },
    storage::error::StorageError,
};

pub type InMemoryType = Arc<RwLock<HashMap<String, String>>>;

#[derive(Clone)]
pub struct InMemoryRepository {
    store: InMemoryType,
}

impl InMemoryRepository {
    pub fn new(store: InMemoryType) -> Self {
        Self { store }
    }
}

#[async_trait]
impl GenerateShortUrlRepository for InMemoryRepository {
    async fn save(&self, short_url: String, full_url: String) -> Result<(), StorageError> {
        self.store.write()?.insert(short_url, full_url);
        Ok(())
    }
}

#[async_trait]
impl GetFullUrlRepository for InMemoryRepository {
    async fn get(&self, short_url: &str) -> Result<String, StorageError> {
        match self.store.read()?.get(short_url) {
            Some(full_url) => Ok(full_url.clone()),
            None => Err(StorageError::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock},
    };

    use crate::{
        app::query::get_full_url::GetFullUrlRepository,
        storage::{error::StorageError, in_memory::InMemoryRepository},
    };

    #[tokio::test]
    async fn get_lock_error() {
        let store = Arc::new(RwLock::new(HashMap::new()));
        let store_in = store.clone();
        let repo = InMemoryRepository::new(store.clone());
        let _ = tokio::spawn(async move {
            let mut r = store_in.write().unwrap();
            r.insert("".to_owned(), "".to_owned());
            panic!();
        })
        .await;

        let res = repo.get("123").await;
        assert!(matches!(res, Err(StorageError::LockError(_))));
    }
}
