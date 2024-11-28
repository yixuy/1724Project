pub mod db {
    pub mod operations;
}
pub mod error;
pub mod models {
    pub mod user;
}
pub mod routes {
    pub mod auth;
    pub mod status;
}
pub mod services {
    pub mod auth_service;
    pub mod hash;
    pub mod jwt;
    pub mod status_service;
    pub mod validation;
}
