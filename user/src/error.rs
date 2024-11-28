use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use crate::routes::auth::Response;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Username must be between 3 and 20 characters.")]
    UsernameInvalidLength,
    #[error("Username can only contain letters and numbers.")]
    UsernameInvalidFormat,
    #[error("Password must be between 8 and 64 characters.")]
    PasswordInvalidLength,
    #[error("Password must contain at least one uppercase letter.")]
    PasswordMissingUppercase,
    #[error("Password must contain at least one lowercase letter.")]
    PasswordMissingLowercase,
    #[error("Password must contain at least one number.")]
    PasswordMissingNumber,
    #[error("Username cannot be empty.")]
    UsernameEmpty,
    #[error("Password cannot be empty.")]
    PasswordEmpty,
}
#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password hashing failed.")]
    HashingError,
    #[error("Password verification failed.")]
    VerificationError,
    #[error("Password invalid.")]
    PasswordInvalid,
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Token generation failed.")]
    TokenGenerationError,
    #[error("Token expired.")]
    TokenExpired,
    #[error("Token invalid")]
    TokenInvalid,
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Username does not exists.")]
    UserNotFound,
    #[error("Username already exists.")]
    UsernameExists,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Failed to update user status.")]
    StatusUpdateError,
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
    #[error(transparent)]
    PasswordError(#[from] PasswordError),
    #[error(transparent)]
    TokenError(#[from] TokenError),
    #[error(transparent)]
    InputError(#[from] InputError),
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::StatusUpdateError => {
                HttpResponse::InternalServerError().json(Response {
                    message: "An unexpected error occurred.".to_string(),
                })
            }
            AuthError::InputError(err) => {
                HttpResponse::BadRequest().json(Response {
                    message: err.to_string(),
                })
            }
            AuthError::PasswordError(err) => match err {
                PasswordError::PasswordInvalid => {
                    HttpResponse::Unauthorized().json(Response {
                        message: "Invalid username or password.".to_string(),
                    })
                }
                _ => {
                    HttpResponse::InternalServerError().json(Response {
                        message: "An unexpected error occurred.".to_string(),
                    })
                }
            },
            AuthError::TokenError(err) => match err {
                TokenError::TokenExpired | TokenError::TokenInvalid => {
                    HttpResponse::Unauthorized().json(Response {
                        message: err.to_string(),
                    })
                }
                _ => {
                    HttpResponse::InternalServerError().json(Response {
                        message: "An unexpected error occurred.".to_string(),
                    })
                }
            },
            AuthError::DatabaseError(err) => match err {
                DatabaseError::UserNotFound | DatabaseError::UsernameExists => {
                    HttpResponse::BadRequest().json(Response {
                        message: err.to_string(),
                    })
                }
            },
        }
    }
}