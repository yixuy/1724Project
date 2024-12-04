use crate::models::user::User;
use gloo::net::websocket::Message;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct RoomId {
    pub room_id: String,
}

#[derive(Properties, PartialEq)]
pub struct RoomProps {
    pub room_id: String,
    pub users: Vec<User>,
    pub messages: Vec<Message>,
}
