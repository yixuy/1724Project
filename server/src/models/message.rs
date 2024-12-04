use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize, Clone)]
// #[serde(crate = "rocket::serde")]
pub struct Message {
    pub room_id: String,
    pub username: String,
    pub message: String,
}
