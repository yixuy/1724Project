use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct NewUser {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,
    #[validate(length(min = 10, message = "Password must be at least 10 characters long"))]
    pub password: String,



}
