use std::net::SocketAddr;

use tokio::net::TcpListener;

use api::init_logging;
use axum::{routing::{get, post}, Router};

mod auth;
mod model;

#[tokio::main]
async fn main() {
    init_logging();

    let app = Router::new()
        .route("/", get(index))
        .route("/login", post(auth::login));

    let address = SocketAddr::from(([127, 0, 0, 1], 5000));
    tracing::debug!("Server listening on {address}...");

    let listener = TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Status: online."
}
