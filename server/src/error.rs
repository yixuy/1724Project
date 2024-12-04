use actix_web::{
    http::{header::HeaderValue, header::CONTENT_TYPE, StatusCode},
    HttpResponse, ResponseError,
};
use thiserror::Error;

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header((CONTENT_TYPE, HeaderValue::from_static("text/plain")))
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::UserCreationFailed => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::NoUserFound => StatusCode::NOT_FOUND,
            UserError::NoSuchUser => StatusCode::NOT_FOUND,
            UserError::UserUpdateFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
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
    #[error("The user is not found")]
    NoSuchUser = 2,
    #[error("The user can not be updated")]
    UserUpdateFailed = 3,
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
    RoomUpdateFailed = 3,
}
// Define the Response struct
#[derive(serde::Serialize)]
struct Response {
    message: String,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::StatusUpdateError => HttpResponse::InternalServerError().json(Response {
                message: "An unexpected error occurred.".to_string(),
            }),
            AuthError::InputError(err) => HttpResponse::BadRequest().json(Response {
                message: err.to_string(),
            }),
            AuthError::PasswordError(err) => match err {
                PasswordError::PasswordInvalid => HttpResponse::Unauthorized().json(Response {
                    message: "Invalid username or password.".to_string(),
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
            AuthError::DatabaseError(err) => match err {
                DatabaseError::UserNotFound | DatabaseError::UsernameExists => {
                    HttpResponse::BadRequest().json(Response {
                        message: err.to_string(),
                    })
                }
            },
            AuthError::UserError(err) => match err {
                UserError::UserCreationFailed
                | UserError::NoUserFound
                | UserError::NoSuchUser
                | UserError::UserUpdateFailed => HttpResponse::BadRequest().json(Response {
                    message: err.to_string(),
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
