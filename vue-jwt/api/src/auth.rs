use axum::response::Json;
use axum::http::StatusCode;
use chrono::Local;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::model::{Credentials, Claims, Tokens};

const USERNAME: &str = "admin";
const PASSWORD: &str = "123";

const ACCESS_TOKEN_EXPIRATION: i64 = 5 * 60;
const REFRESH_TOKEN_EXPIRATION: i64 = 30 * 60;

pub async fn login(Json(credentials): Json<Credentials>) -> Result<Json<Tokens>, (StatusCode, String)> {
    if credentials.username.is_empty() || credentials.password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, String::from("Username and password are mandatory.")));
    }

    if credentials.username == USERNAME && credentials.password == PASSWORD {
        let now = Local::now().timestamp();
        let iat = usize::try_from(now).unwrap();
        let access_exp = usize::try_from(now + ACCESS_TOKEN_EXPIRATION).unwrap();
        let refresh_exp = usize::try_from(now + REFRESH_TOKEN_EXPIRATION).unwrap();

        let access_claims = Claims { exp: access_exp, iat, iss: String::from("MY_RUST_API"), sub: String::from(USERNAME)};
        let refresh_claims = Claims { exp: refresh_exp, iat, iss: String::from("MY_RUST_API"), sub: String::from(USERNAME)};

        let token = Tokens{
            access_token: encode(&Header::default(), &access_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap(),
            refresh_token: encode(&Header::default(), &refresh_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap(),
        };

        return Ok(Json(token));
    }

    return Err((StatusCode::BAD_REQUEST, String::from("Invalid username/password.")));
}
