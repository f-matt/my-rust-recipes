use axum::response::Json;
use axum::http::StatusCode;

use crate::model::{Credentials, Tokens};


pub async fn login(Json(credentials): Json<Credentials>) -> Result<Json<Tokens>, (StatusCode, String)> {
    Err((StatusCode::BAD_REQUEST, String::from("Under construction...")))
}
