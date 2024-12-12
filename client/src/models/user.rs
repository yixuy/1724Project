use serde::{Deserialize, Serialize};
use std::fmt;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub enum UserStatus {
    #[default]
    Online,
    Offline,
    Leave,
}
impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            UserStatus::Online => "Online",
            UserStatus::Offline => "Offline",
            UserStatus::Leave => "Leave",
        };
        write!(f, "{}", status_str)
    }
}
#[derive(Validate, Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct User {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,
    #[validate(length(min = 5, message = "Password must be at least 10 characters long"))]
    pub password: String,
    pub status: UserStatus,
    pub room_id: Option<String>,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User {
            username,
            password,
            status: UserStatus::Offline,
            room_id: None,
        }
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    // pub fn set_password(&mut self, password: String) {
    //     self.password = password;
    // }

    // pub fn set_status(&mut self, status: bool) {
    //     self.status = status;
    // }
}
