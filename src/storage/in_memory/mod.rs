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
        self.store
            .write()
            .map_err(|_| StorageError::LockError)?
            .insert(short_url, full_url);
        Ok(())
    }
}

#[async_trait]
impl GetFullUrlRepository for InMemoryRepository {
    async fn get(&self, short_url: &str) -> Result<String, StorageError> {
        match self
            .store
            .read()
            .map_err(|_| StorageError::LockError)?
            .get(short_url)
        {
            Some(full_url) => Ok(full_url.clone()),
            None => Err(StorageError::NotFound),
        }
    }
}
