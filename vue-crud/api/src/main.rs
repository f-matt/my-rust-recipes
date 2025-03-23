use axum::{
    routing::{get, post, patch, delete},
    Router,
};

use tokio::net::TcpListener;

use api::*;
use std::net::SocketAddr;

use crate::repository::insert_foo;
use crate::repository::get_foos;
use crate::repository::update_foo;
use crate::repository::delete_foo;

#[tokio::main]
async fn main() {
    init_logging();
    let pool = setup_connection_pool();

    let app = Router::new()
        .route("/", get(index))
        .route("/foos", get(get_foos))
        .route("/create-foo", post(insert_foo))
        .route("/update-foo", patch(update_foo))
        .route("/delete-foo", delete(delete_foo))
        .with_state(pool);

    let address = SocketAddr::from(([127, 0, 0, 1], 5000));
    tracing::debug!("Server listening on {address}");
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn index() -> &'static str {
    "Status: online."
}


