use actix_web::{
    http::{header::HeaderValue, header::CONTENT_TYPE, StatusCode},
    HttpResponse, ResponseError,
};
use thiserror::Error;

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::UserCreationInvalid => StatusCode::BAD_REQUEST,
            UserError::UsernameExists => StatusCode::BAD_REQUEST,
            UserError::UserCreationFailed => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::NoUserFound => StatusCode::NOT_FOUND,
            UserError::NoSuchUser => StatusCode::NOT_FOUND,
            UserError::UserSearchFailed => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::UserUpdateFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header((CONTENT_TYPE, HeaderValue::from_static("text/plain")))
            .body(self.to_string())
    }
}

impl ResponseError for RoomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header((CONTENT_TYPE, HeaderValue::from_static("text/plain")))
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            RoomError::RoomCreationFailed => StatusCode::INTERNAL_SERVER_ERROR,
            RoomError::NoRoomFound => StatusCode::NOT_FOUND,
            RoomError::NoSuchRoom => StatusCode::NOT_FOUND,
            RoomError::RoomUpdateFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
#[derive(Debug, Error)]
#[allow(dead_code)]
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
    #[error("Invalid password.")]
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
pub enum AuthError {
    #[error(transparent)]
    PasswordError(#[from] PasswordError),
    #[error(transparent)]
    TokenError(#[from] TokenError),
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    RoomError(#[from] RoomError),
}
#[derive(Debug, Error)]
pub enum UserError {
    #[error("User creation failed")]
    UserCreationFailed = 0,
    #[error("No user found")]
    NoUserFound = 1,
    #[error("User does not exist")]
    NoSuchUser = 2,
    #[error("The user can not be updated")]
    UserUpdateFailed = 3,
    #[error("Username or password too short")]
    UserCreationInvalid,
    #[error("Username already exists")]
    UsernameExists,
    #[error("User search failed")]
    UserSearchFailed,
}

#[derive(Debug, Error)]
pub enum RoomError {
    #[error("Room creation failed")]
    RoomCreationFailed = 0,
    #[error("No room found")]
    NoRoomFound = 1,
    #[error("The room is not found")]
    NoSuchRoom = 2,
    #[error("The room can not be updated")]
    RoomUpdateFailed,
}
// Define the Response struct
#[derive(serde::Serialize)]
struct Response {
    message: String,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::PasswordError(err) => match err {
                PasswordError::PasswordInvalid => HttpResponse::Unauthorized().json(Response {
                    message: err.to_string(),
                }),
                _ => HttpResponse::InternalServerError().json(Response {
                    message: "An unexpected error occurred.".to_string(),
                }),
            },
            AuthError::TokenError(err) => match err {
                TokenError::TokenExpired | TokenError::TokenInvalid => HttpResponse::Unauthorized()
                    .json(Response {
                        message: err.to_string(),
                    }),
                _ => HttpResponse::InternalServerError().json(Response {
                    message: "An unexpected error occurred.".to_string(),
                }),
            },
            AuthError::UserError(err) => match err {
                UserError::NoUserFound | UserError::NoSuchUser => {
                    HttpResponse::BadRequest().json(Response {
                        message: err.to_string(),
                    })
                }
                UserError::UserCreationInvalid | UserError::UsernameExists => {
                    HttpResponse::BadRequest().json(Response {
                        message: err.to_string(),
                    })
                }
                _ => HttpResponse::InternalServerError().json(Response {
                    message: "An unexpected error occurred.".to_string(),
                }),
            },
            AuthError::RoomError(err) => match err {
                RoomError::RoomCreationFailed
                | RoomError::NoRoomFound
                | RoomError::NoSuchRoom
                | RoomError::RoomUpdateFailed => HttpResponse::BadRequest().json(Response {
                    message: err.to_string(),
                }),
            },
        }
    }
}
