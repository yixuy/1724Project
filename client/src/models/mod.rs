// This file include all the components that will be used in the frontend
pub mod room;
pub mod user;
// pub mod user_trait;

pub mod prelude {
    pub use super::room::RoomProps;
    pub use super::user::User;
    // pub use super::user_trait::UserTrait;
}
