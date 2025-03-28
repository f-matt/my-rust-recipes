use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use axum::{extract::Request, response::Json};
use axum::http::{HeaderMap, StatusCode};
use chrono::Local;
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};

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

pub async fn has_token(headers: HeaderMap, request: Request, next: Next) -> Result<Response, (StatusCode, String)> {
    match get_token(&headers) {
        Some(token) if token_is_valid(&token) => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => {
            Err((StatusCode::UNAUTHORIZED, String::from("Access denied.")))
        }
    }
}

fn get_token(headers: &HeaderMap) -> Option<String> {
    let token = headers.get(AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        }
    );

    token
}

fn token_is_valid(token: &String) -> bool {
    decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).is_ok()
}
