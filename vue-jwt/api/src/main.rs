use std::net::SocketAddr;

use tokio::net::TcpListener;

use api::init_logging;
use axum::{middleware, routing::{get, post}, Router};

mod auth;
mod model;

#[tokio::main]
async fn main() {
    init_logging();

    let app = Router::new()
        .route("/protected", get(protected))
        .route_layer(middleware::from_fn(auth::has_token))
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

async fn protected() -> &'static str {
    "Access granted."
}
