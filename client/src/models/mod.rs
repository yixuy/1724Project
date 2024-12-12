// This file include all the components that will be used in the frontend
pub mod message;
pub mod room;
pub mod user;

pub mod prelude {
    pub use super::message::NewChatMessage;
    pub use super::message::ChatMessage;
    pub use super::room::Room;
    pub use super::room::RoomAttribute;
    pub use super::user::User;
}
