use serde::{Deserialize, Serialize};
use yew::Properties;
#[derive(Properties, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NewChatMessage {
    pub username: String,
    pub content: String,
}

#[derive(Properties, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub username: String,
    // pub user_status: UserStatus,
    pub content: String,
    pub timestamp: String,
}

#[derive(Properties, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct DisplayMessage {
    pub username: String,
    pub user_status: String,
    pub content: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct LeaveRoomMessage {
    pub username: String,
    pub room_id: String,
}
