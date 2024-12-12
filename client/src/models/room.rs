use crate::models::message::NewChatMessage;
use serde::{Deserialize, Serialize};
use validator::Validate;
use yew::Properties;

#[derive(Properties, PartialEq, Validate, Clone, Debug, Serialize, Deserialize)]
pub struct RoomAttribute {
    pub username: String,
    #[validate(length(min = 3, message = "Room number must be at least 3 characters long"))]
    // #[validate(regex(path = "NUMERIC_REGEX", message = "Room number must be numeric"))]
    pub room_id: String,
}
// lazy_static::lazy_static! {
//     static ref NUMERIC_REGEX: regex::Regex = regex::Regex::new(r"^\d+$").unwrap();
// }
#[derive(Validate, Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Room {
    pub room_id: String,
    pub users: Vec<String>,
    pub messages: Vec<NewChatMessage>,
}

impl Room {
    pub fn new(room_id: String) -> Room {
        Room {
            room_id,
            users: Vec::new(),
            messages: Vec::new(),
        }
    }
}
