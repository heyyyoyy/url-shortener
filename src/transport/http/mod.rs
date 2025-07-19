use axum::{
    Json, Router,
    extract::{MatchedPath, Path, Request, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::app::{
    App,
    command::generate_url::{GenerateShortUrlRepository, ShortUrlProvider},
    query::get_full_url::GetFullUrlRepository,
};

#[derive(Serialize, Deserialize)]
struct GenerateShortUrlRequest {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct GenerateShortUrlResponse {
    short_url: String,
}

#[derive(Serialize, Deserialize)]
struct GetFullUrlResponse {
    full_url: String,
}

pub struct Server<P, R, Q>
where
    P: ShortUrlProvider + Send + Sync + 'static,
    R: GenerateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    app: Arc<App<P, R, Q>>,
    port: u16,
}

impl<P, R, Q> Server<P, R, Q>
where
    P: ShortUrlProvider + Send + Sync + 'static,
    R: GenerateShortUrlRepository + Send + Sync + 'static,
    Q: GetFullUrlRepository + Send + Sync + 'static,
{
    pub fn new(app: Arc<App<P, R, Q>>, port: u16) -> Self {
        Self { app, port }
    }

    pub async fn run(&self) {
        self.setup_tracer();
        let router = self.get_router();
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        tracing::info!("Listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, router).await.unwrap();
    }

    fn setup_tracer(&self) {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
                }),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    fn get_router(&self) -> Router {
        Router::new()
            .route("/", post(Server::generate_short_url))
            .route("/{short_url}", get(Server::get_full_url))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|req: &Request| {
                        let method = req.method();
                        let matched_path = req
                            .extensions()
                            .get::<MatchedPath>()
                            .map(MatchedPath::as_str);
                        let uri = req.uri();

                        tracing::debug_span!(
                            "http_request",
                            %method,
                            %uri,
                            matched_path,
                        )
                    })
                    .on_failure(()),
            )
            .with_state(self.app.clone())
    }

    async fn generate_short_url(
        State(app): State<Arc<App<P, R, Q>>>,
        Json(params): Json<GenerateShortUrlRequest>,
    ) -> Result<Json<GenerateShortUrlResponse>, String> {
        app.generate_short_url
            .generate(&params.url)
            .await
            .map(|short_url| Json(GenerateShortUrlResponse { short_url }))
    }

    async fn get_full_url(
        State(app): State<Arc<App<P, R, Q>>>,
        Path(short_url): Path<String>,
    ) -> Result<Json<GetFullUrlResponse>, String> {
        app.get_full_url
            .get(&short_url)
            .await
            .map(|full_url| Json(GetFullUrlResponse { full_url }))
    }
}
