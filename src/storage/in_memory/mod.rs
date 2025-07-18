use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;

use crate::app::{
    command::generate_url::GenerateShortUrlRepository, query::get_full_url::GetFullUrlRepository,
};

pub type InMemoryType = Arc<RwLock<HashMap<String, String>>>;

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
    async fn save(&self, id: String, full_url: String) -> Result<(), String> {
        self.store
            .write()
            .map_err(|_| "Storage lock error".to_owned())?
            .insert(id, full_url);
        Ok(())
    }
}

#[async_trait]
impl GetFullUrlRepository for InMemoryRepository {
    async fn get(&self, short_url: &str) -> Result<String, String> {
        match self
            .store
            .read()
            .map_err(|_| "Storage lock error".to_owned())?
            .get(short_url)
        {
            Some(full_url) => Ok(full_url.clone()),
            None => Err("Url not found".to_owned()),
        }
    }
}
