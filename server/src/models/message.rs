// use crate::models::user::UserStatus;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Validate, Debug, Serialize, Deserialize, Clone)]

pub struct NewChatMessage {
    pub username: String,
    pub content: String,
}
#[derive(Validate, Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub timestamp: String,
    pub username: String,
    pub content: String,
    // pub user_status: UserStatus,
}
