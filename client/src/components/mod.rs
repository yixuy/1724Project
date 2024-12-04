// This file include all the components that will be used in the frontend
pub mod home;
pub mod logout;
pub mod notfound;
pub mod room;
pub mod signin;
pub mod signup;

pub mod prelude {
    pub use super::home::Home;
    pub use super::logout::LogOut;
    pub use super::notfound::NotFound;
    pub use super::room::Room;
    pub use super::signin::SignIn;
    pub use super::signup::SignUp;
}
