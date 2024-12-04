use crate::models::message::Message; // Add this line to import the Message type
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use validator::Validate; // Add this line to import the User type

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct Room {
    pub room_id: String,
    pub users: Vec<User>,
    pub messages: Vec<Message>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NewRoom {
    pub room_id: String,
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
