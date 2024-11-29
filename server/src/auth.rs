// use actix_web::{web, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};
// use crate::error::{AuthError, InputError};
// use crate::services::{auth_service, validation};

// /* User authentication route handlers */

// #[derive(Deserialize)]
// pub struct UserInput {
//     username: String,
//     password: String,
// }

// #[derive(Serialize)]
// pub struct Response {
//     pub message: String,
// }

// #[derive(Serialize, Deserialize)]
// pub struct TokenData {
//     token: String,
// }

// /* Status Code Returned:
//     - 200 ok for successful registration or login
//     - 400 bad request for missing or invalid input
//     - 401 unauthorized for login failures
//     - 500 internal server error for unexpected errors
// */
// pub async fn register(data: web::Json<UserInput>) -> impl Responder {
//     // validates user input
//     validation::validate_username(&data.username)?;
//     validation::validate_password(&data.password)?;

//     // calls auth_service::register_user() to handle registration logic
//     auth_service::register_user(&data.username, &data.password).await?;

//     Ok(HttpResponse::Ok().json(Response {
//         message: "User registered successfully.".to_string(),
//     }))
// }

// pub async fn login(data: web::Json<UserInput>) -> impl Responder {
//     // validates user credentials
//     if data.username.is_empty() {
//         return Err(AuthError::InputError(InputError::UsernameEmpty));
//     }
//     if data.password.is_empty() {
//         return Err(AuthError::InputError(InputError::PasswordEmpty));
//     }

//     // calls auth_service::login_user() to handle authentication logic
//     let token = auth_service::login_user(&data.username, &data.password).await?;
//     Ok(HttpResponse::Ok().json(TokenData { token }))
// }

// pub async fn logout(data: web::Json<TokenData>) -> impl Responder {
//     auth_service::logout_user(&data.token).await?;
//     Ok(HttpResponse::Ok().json(Response {
//         message: "Logout successfully.".to_string(),
//     }))
// }
