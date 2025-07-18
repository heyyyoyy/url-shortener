use async_trait::async_trait;

use crate::adapters::id_provider::IdProvider;

#[async_trait]
pub trait GenerateShortUrlRepository {
    async fn save(&self, id: String, full_url: String) -> Result<(), String>;
}
pub struct GenerateShortUrl<P, R>
where
    P: IdProvider,
    R: GenerateShortUrlRepository,
{
    id_provider: P,
    repository: R,
}

impl<P, R> GenerateShortUrl<P, R>
where
    P: IdProvider,
    R: GenerateShortUrlRepository,
{
    pub fn new(id_provider: P, repository: R) -> Self {
        Self {
            id_provider,
            repository,
        }
    }
    async fn generate(&self, full_url: String) -> Result<String, String> {
        let id = self.id_provider.provide();

        self.repository.save(id.clone(), full_url).await?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock},
    };

    use crate::{
        adapters::id_provider::{FakeIdProvider, NanoIdProvider},
        storage::in_memory::InMemoryRepository,
    };

    use super::GenerateShortUrl;

    #[tokio::test]
    async fn get_non_empty_short_url() {
        let fake_id_provider = FakeIdProvider::new("1".to_owned());
        let store = Arc::new(RwLock::new(HashMap::new()));
        let repository = InMemoryRepository::new(store);
        let command = GenerateShortUrl::new(fake_id_provider, repository);

        let short_url = command
            .generate("https://youtube.com".to_owned())
            .await
            .unwrap();

        assert_ne!(short_url, "".to_owned());
    }

    #[tokio::test]
    async fn comparing_different_urls() {
        let nanoid_provider = NanoIdProvider;
        let store = Arc::new(RwLock::new(HashMap::new()));
        let repository = InMemoryRepository::new(store);
        let command = GenerateShortUrl::new(nanoid_provider, repository);

        let short_url = command
            .generate("https://youtube.com".to_owned())
            .await
            .unwrap();
        let short_url_two = command
            .generate("https://google.com".to_owned())
            .await
            .unwrap();

        assert_ne!(short_url, short_url_two);
    }
    #[tokio::test]
    async fn check_non_empty_store() {
        let nanoid_provider = NanoIdProvider;
        let store = Arc::new(RwLock::new(HashMap::new()));
        let repository = InMemoryRepository::new(store.clone());
        let command = GenerateShortUrl::new(nanoid_provider, repository);

        let id = command
            .generate("https://youtube.com".to_owned())
            .await
            .unwrap();

        assert_eq!(store.read().unwrap().len(), 1);

        assert_eq!(
            store.read().unwrap().get(&id).unwrap(),
            "https://youtube.com"
        );
    }
}
