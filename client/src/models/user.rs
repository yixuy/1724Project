use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,
    #[validate(length(min = 5, message = "Password must be at least 10 characters long"))]
    pub password: String,
    pub status: bool,
    pub room_id: Option<String>,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User {
            username,
            password,
            status: false,
            room_id: None,
        }
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn set_status(&mut self, status: bool) {
        self.status = status;
    }
}
