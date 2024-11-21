use actix_web::{
    http::{header::HeaderValue, header::CONTENT_TYPE, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;
#[derive(Debug, Display)]
pub enum UserError {
    #[display("User creation failed")]
    UserCreationFailed = 0,
    #[display("No user found")]
    NoUserFound = 1,
    #[display("The user is not found")]
    NoSuchUser = 2,
}

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
        }
    }
}
