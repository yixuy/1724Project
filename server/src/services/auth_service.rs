// use crate::db::operations;
use crate::error::{AuthError, PasswordError, TokenError};
use crate::services::status_service;
use crate::services::{hash, jwt};

// /* User authentication-related logics */
// pub async fn register_user(username: &str, password: &str) -> Result<(), AuthError> {
//     // hash the password
//     let hashed = hash::hash_password(password.)?;

//     // store the user in the database
//     operations::create_user(username, &hashed).await?;

//     Ok(())
// }

// pub async fn login_user(username: &str, password: &str) -> Result<String, AuthError> {
//     // retrieve user from the database
//     let user = operations::get_user_by_username(username).await?;

//     // verifies credentials
//     hash::verify_password(password, &user.hashed_password)?;

//     // update the user's status to online
//     status_service::set_online(username).await.map_err(|err| {
//         eprintln!("Error updating user status to online: {:?}", err);
//         AuthError::StatusUpdateError
//     })?;

//     // generate and return a JWT token
//     let token = jwt::generate_token(username)?;
//     Ok(token)
// }



pub async fn verify_auth_token(token: &str) -> Result<String, AuthError> {
    // verifies JWT for protected routes
    let claims = jwt::verify_token(token).await?;
    Ok(claims.sub)
}
