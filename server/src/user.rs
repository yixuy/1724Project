use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct NewUser {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,
    #[validate(length(min = 5, message = "Password must be at least 10 characters long"))]
    pub password: String,
    // pub uuid: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct UpdateUserURL {
    pub uuid: String,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(uuid: String, username: String, password: String) -> User {
        User {
            uuid,
            username,
            password,
        }
    }
}
