use crate::app::{
    command::generate_url::{
        GenerateShortUrlCommand, GenerateShortUrlRepository, ShortUrlProvider,
    },
    query::get_full_url::{GetFullUrlQuery, GetFullUrlRepository},
};

pub mod command;
pub mod query;

pub struct App<P, R, Q>
where
    P: ShortUrlProvider,
    R: GenerateShortUrlRepository,
    Q: GetFullUrlRepository,
{
    pub generate_short_url: GenerateShortUrlCommand<P, R>,
    pub get_full_url: GetFullUrlQuery<Q>,
}

impl<P, R, Q> App<P, R, Q>
where
    P: ShortUrlProvider,
    R: GenerateShortUrlRepository,
    Q: GetFullUrlRepository,
{
    pub fn new(short_url_provider: P, command_repo: R, query_repo: Q) -> Self {
        Self {
            generate_short_url: GenerateShortUrlCommand::new(short_url_provider, command_repo),
            get_full_url: GetFullUrlQuery::new(query_repo),
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
        adapters::short_url_provider::NanoShortUrl, app::App,
        storage::in_memory::InMemoryRepository,
    };

    #[tokio::test]
    async fn generate_short_get_full_url() {
        let url_provider = NanoShortUrl;
        let store = Arc::new(RwLock::new(HashMap::new()));
        let repo = InMemoryRepository::new(store);
        let app = App::new(url_provider, repo.clone(), repo);

        let full_url = "https://youtube.com".to_owned();
        let short_url = app
            .generate_short_url
            .generate(full_url.clone())
            .await
            .unwrap();
        let full_url_from_repo = app.get_full_url.get(&short_url).await.unwrap();
        assert_eq!(full_url, full_url_from_repo);
    }
}
