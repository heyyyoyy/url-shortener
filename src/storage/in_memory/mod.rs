use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;

use crate::app::command::generate_url::GenerateShortUrlRepository;

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
