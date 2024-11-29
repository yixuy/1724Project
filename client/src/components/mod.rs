// This file include all the components that will be used in the frontend
pub mod logout;
pub mod notfound;
pub mod room;
pub mod signin;
pub mod signup;
pub mod home;
pub mod common;

pub mod prelude {
    pub use super::logout::LogOut;
    pub use super::notfound::NotFound;
    pub use super::room::Room;
    pub use super::signin::SignIn;
    pub use super::signup::SignUp;
    pub use super::home::Home;
}
