pub mod auth_service;
pub mod hash;
pub mod jwt;
pub mod status_service;
// pub mod validation;

pub mod prelude {
    // pub use super::auth_service::{login_user, logout_user, register_user, verify_auth_token};
    pub use super::auth_service::verify_auth_token;
    pub use super::hash::{hash_password, verify_password};
    pub use super::jwt;
    pub use super::jwt::{generate_token, verify_token};
    // pub use super::status_service::{set_offline, set_online};
    // pub use super::validation::{validate_password, validate_username};
}
