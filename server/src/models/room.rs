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
