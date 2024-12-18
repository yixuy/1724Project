// This file include all the components that will be used in the frontend
pub mod message;
pub mod room;
pub mod room_trait;
pub mod user;
pub mod user_trait;

pub mod prelude {
    pub use super::message::ChatMessage;
    pub use super::message::NewChatMessage;
    pub use super::room_trait::RoomTrait;
}
