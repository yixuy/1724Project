use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::ErrorKind;
use serde::{Deserialize, Serialize};
use crate::error::TokenError;

/* JWT-related utilities */

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

const SECRET: &str = "secret";  // placeholder; consider load from env vars

pub fn generate_token(username: &str) -> Result<String, TokenError> {
    // creates a JWT for authenticated users
    let expiration_time = Utc::now()
        .checked_add_signed(Duration::hours(1))
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

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_ref()))
        .map_err(|err| {
            eprintln!("Failed to generate token: {:?}", err);
            TokenError::TokenGenerationError
        })
}

pub fn verify_token(token: &str) -> Result<Claims, TokenError> {
    // validates and decodes a JWT
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET.as_ref()),
                           &Validation::default())
        .map(|data| data.claims)
        .map_err(|err| match *err.kind() {
            ErrorKind::ExpiredSignature => TokenError::TokenExpired,
            _ => {
                eprintln!("Failed to verify token: {:?}", err);
                TokenError::TokenInvalid
            },
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_token() {
        let username = "admin123";
        let token = generate_token(username).expect("Token generation failed.");
        let claims = verify_token(&token).expect("Token should be valid.");
        assert_eq!(claims.sub, username, "Usernames should match.");
    }

    #[test]
    fn test_verify_token_expired() {
        let username = "admin123";
        let claims = Claims {
            sub: username.to_string(),
            exp: (Utc::now() - Duration::hours(2)).timestamp(),
        };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_ref()))
            .expect("Token generation failed.");
        let result = verify_token(&token);
        assert!(result.is_err(), "Token should have expired");
    }

    #[test]
    fn test_verify_token_invalid() {
        let invalid_token = "invalid_token";
        let result = verify_token(invalid_token);
        assert!(result.is_err(), "Token should not be valid.");
    }
}
