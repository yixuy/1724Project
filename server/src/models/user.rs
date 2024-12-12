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

#[derive(Validate, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Validate, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub status: UserStatus,
}
// }
// #[derive(Validate, Debug, Serialize, Deserialize, Clone)]
// pub struct UserMessage {
//     pub username: String,
//     pub status: UserStatus,
// }

// impl UserMessage {
//     pub fn new(username: String, status: UserStatus) -> UserMessage {
//         UserMessage { username, status }
//     }
// }

impl User {
    pub fn new(uuid: String, username: String, password: String, status: UserStatus) -> User {
        User {
            uuid,
            username,
            password,
            status,
        }
    }
}
