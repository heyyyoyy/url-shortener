use async_trait::async_trait;

#[async_trait]
pub trait GetFullUrlRepository {
    async fn get(&self, short_url: &str) -> Result<String, String>;
}

pub struct GetFullUrlQuery<R>
where
    R: GetFullUrlRepository,
{
    repository: R,
}

impl<R> GetFullUrlQuery<R>
where
    R: GetFullUrlRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get(&self, short_url: &str) -> Result<String, String> {
        self.repository.get(short_url).await
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, RwLock},
    };

    use async_trait::async_trait;

    use crate::{
        app::query::get_full_url::{GetFullUrlQuery, GetFullUrlRepository},
        storage::in_memory::InMemoryRepository,
    };

    #[tokio::test]
    async fn get_full_url() {
        struct FakeRepo;

        #[async_trait]
        impl GetFullUrlRepository for FakeRepo {
            async fn get(&self, _short_url: &str) -> Result<String, String> {
                Ok("https://youtube.com".to_owned())
            }
        }
        let repo = FakeRepo;
        let query = GetFullUrlQuery::new(repo);
        assert_eq!(
            query.get("some string").await,
            Ok("https://youtube.com".to_owned())
        );
    }
    #[tokio::test]
    async fn get_full_url_from_in_memory_repo() {
        let storage = Arc::new(RwLock::new(HashMap::new()));
        storage
            .write()
            .unwrap()
            .insert("qwerty".to_owned(), "https://youtube.com".to_owned());
        let repo = InMemoryRepository::new(storage);
        let query = GetFullUrlQuery::new(repo);

        assert_eq!(
            query.get("qwerty").await,
            Ok("https://youtube.com".to_owned())
        );
    }
    #[tokio::test]
    async fn get_two_urls() {
        let storage = Arc::new(RwLock::new(HashMap::new()));
        storage.write().unwrap().extend([
            ("qwerty".to_owned(), "https://youtube.com".to_owned()),
            ("qwerty2".to_owned(), "https://google.com".to_owned()),
        ]);
        let repo = InMemoryRepository::new(storage);
        let query = GetFullUrlQuery::new(repo);

        assert_eq!(
            query.get("qwerty").await,
            Ok("https://youtube.com".to_owned())
        );
        assert_eq!(
            query.get("qwerty2").await,
            Ok("https://google.com".to_owned())
        );
    }
}
