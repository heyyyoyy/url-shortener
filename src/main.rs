use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    adapters::short_url_provider::NanoShortUrl, app::App, storage::in_memory::InMemoryRepository,
    transport::http::Server,
};

pub mod adapters;
pub mod app;
pub mod storage;
pub mod transport;

#[tokio::main]
async fn main() {
    let store = Arc::new(RwLock::new(HashMap::new()));
    let short_url_provider = NanoShortUrl;
    let in_memory_repo = InMemoryRepository::new(store);
    let app = Arc::new(App::new(
        short_url_provider,
        in_memory_repo.clone(),
        in_memory_repo,
    ));
    let server = Server::new(app, 8081);
    server.run().await;
}
