pub mod models;
pub mod schema;
pub mod repository;

use axum::http::StatusCode;

use dotenvy::dotenv;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use deadpool_diesel::postgres::{Manager, Pool};

pub fn init_logging() {
   tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub fn setup_connection_pool() -> Pool  {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap();

    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    Pool::builder(manager)
        .build()
        .unwrap()
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

