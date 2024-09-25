use app_context::AppContext;
use axum::{routing::get, Router};
use lightning_ingestor::run_ingestor;

mod app_context;
mod lightning_fetcher;
mod lightning_ingestor;

async fn list_nodes() -> &'static str {
    "data"
}

#[tokio::main]
async fn main() {
    let app_context = AppContext::new();
    let app = Router::new()
        .route("/", get(list_nodes))
        .with_state(app_context.clone());

    let ingestor_handler = run_ingestor(app_context);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    ingestor_handler.abort();
}
