use axum::{routing::get, Json, Router, http::StatusCode};

mod lightning_fetcher;
mod ingestor;

async fn list_nodes() -> &'static str {
    "data"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(list_nodes));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
