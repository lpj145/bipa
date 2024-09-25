use std::env;

use app_context::AppContext;
use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use lightning_ingestor::run_ingestor;
use mongodb::{bson::doc, Client};
use node_data::NodeDocument;
use reqwest::StatusCode;
use tokio::signal;

mod app_context;
mod lightning_fetcher;
mod lightning_ingestor;
mod node_data;

async fn list_nodes(State(context): State<AppContext>) -> impl IntoResponse {
    let node_collection = context.database.collection::<NodeDocument>("nodes");
    let mut result = Vec::with_capacity(100);
    // Here we can panic because it's safe to panic under axum task
    let mut cursor = node_collection
        .find(doc! {})
        .await
        .expect("Unable to get documents from mongodb");

    while cursor
        .advance()
        .await
        .expect("Unable to advance mongo cursor")
    {
        result.push(cursor.deserialize_current().unwrap());
    }

    (StatusCode::OK, Json(result))
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to load env vars...");
    let mongo_client = Client::with_uri_str(env::var("MONGODB_URI").unwrap())
        .await
        .expect("Error during connection with mongodb");
    let app_context = AppContext::new(mongo_client);
    let app = Router::new()
        .route("/nodes", get(list_nodes))
        .with_state(app_context.clone());

    let ingestor_handler = run_ingestor(app_context);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening at: http://127.0.0.1:3000/nodes");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    ingestor_handler.abort();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
