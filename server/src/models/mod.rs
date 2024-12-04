// This file include all the components that will be used in the frontend
pub mod user;
pub mod user_trait;
pub mod room;
pub mod message;


pub mod prelude {
    pub use super::user::User;
    pub use super::user_trait::UserTrait;
    pub use super::room::Room;
    pub use super::message::Message;
}
