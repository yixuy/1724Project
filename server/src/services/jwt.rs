use crate::error::TokenError;
use chrono::{Duration, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/* JWT-related utilities */

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

const SECRET: &str = "secret"; // placeholder; consider load from env vars

pub fn generate_token(username: &str) -> Result<String, TokenError> {
    // creates a JWT for authenticated users
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::hours(12))
        .ok_or_else(|| {
            eprintln!("Failed to compute expiration time.");
            TokenError::TokenGenerationError
        })?
        .timestamp();

    // includes username in the token payload
    let claims = Claims {
        sub: username.to_string(),
        exp: expiration_time,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
    .map_err(|err| {
        eprintln!("Failed to generate token: {:?}", err);
        TokenError::TokenGenerationError
    })
}

pub async fn verify_token(token: &str) -> Result<Claims, TokenError> {
    // validates and decodes a JWT
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|err| match *err.kind() {
        ErrorKind::ExpiredSignature => TokenError::TokenExpired,
        _ => {
            eprintln!("Failed to verify token: {:?}", err);
            TokenError::TokenInvalid
        }
    })
}
