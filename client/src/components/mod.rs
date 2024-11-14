// This file include all the components that will be used in the frontend
pub mod signin;
pub mod signup;
pub mod settings;
pub mod logout;
pub mod notfound;

pub mod prelude {
    pub use super::signin::SignIn;
    pub use super::signup::SignUp;
    pub use super::settings::Settings;
    pub use super::logout::LogOut;
    pub use super::notfound::NotFound;
    
}