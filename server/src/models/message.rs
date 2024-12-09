use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize, Clone)]
// #[serde(crate = "rocket::serde")]
pub struct ChatMessage {
    // pub room_id: String,
    pub username: String,
    pub content: String,
}

