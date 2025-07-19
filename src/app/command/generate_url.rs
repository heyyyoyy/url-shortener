use async_trait::async_trait;

pub trait ShortUrlProvider {
    fn provide(&self) -> String;
}

#[async_trait]
pub trait GenerateShortUrlRepository {
    async fn save(&self, id: String, full_url: String) -> Result<(), String>;
}
pub struct GenerateShortUrlCommand<P, R>
where
    P: ShortUrlProvider,
    R: GenerateShortUrlRepository,
{
    short_url_provider: P,
    repository: R,
}

impl<P, R> GenerateShortUrlCommand<P, R>
where
    P: ShortUrlProvider,
    R: GenerateShortUrlRepository,
{
    pub fn new(short_url_provider: P, repository: R) -> Self {
        Self {
            short_url_provider,
            repository,
        }
    }
    pub async fn generate(&self, full_url: &str) -> Result<String, String> {
        let short_url = self.short_url_provider.provide();

        self.repository
            .save(short_url.clone(), full_url.to_owned())
            .await?;
        Ok(short_url)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock},
    };

    use crate::{
        adapters::short_url_provider::{FakeShortUrl, NanoShortUrl},
        storage::in_memory::InMemoryRepository,
    };

    use super::GenerateShortUrlCommand;

    #[tokio::test]
    async fn get_non_empty_short_url() {
        let fake_id_provider = FakeShortUrl::new("1".to_owned());
        let store = Arc::new(RwLock::new(HashMap::new()));
        let repository = InMemoryRepository::new(store);
        let command = GenerateShortUrlCommand::new(fake_id_provider, repository);

        let short_url = command.generate("https://youtube.com").await.unwrap();

        assert_ne!(short_url, "".to_owned());
    }

    #[tokio::test]
    async fn comparing_different_urls() {
        let nanoid_provider = NanoShortUrl;
        let store = Arc::new(RwLock::new(HashMap::new()));
        let repository = InMemoryRepository::new(store);
        let command = GenerateShortUrlCommand::new(nanoid_provider, repository);

        let short_url = command.generate("https://youtube.com").await.unwrap();
        let short_url_two = command.generate("https://google.com").await.unwrap();

        assert_ne!(short_url, short_url_two);
    }
    #[tokio::test]
    async fn check_non_empty_store() {
        let nanoid_provider = NanoShortUrl;
        let store = Arc::new(RwLock::new(HashMap::new()));
        let repository = InMemoryRepository::new(store.clone());
        let command = GenerateShortUrlCommand::new(nanoid_provider, repository);

        let id = command.generate("https://youtube.com").await.unwrap();

        assert_eq!(store.read().unwrap().len(), 1);

        assert_eq!(
            store.read().unwrap().get(&id).unwrap(),
            "https://youtube.com"
        );
    }
}
