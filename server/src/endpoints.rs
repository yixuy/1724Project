use crate::db;
use crate::error;
// use crate::user;
// use crate::user_trait::UserTrait;
use crate::models::user;
use crate::models::user_trait::UserTrait;
use crate::services::jwt;
use actix_web::web::Data;
use actix_web::{get, post, web::Json, web::Path, HttpResponse, Responder};
use db::Database;
use error::UserError;
use user::NewUser;
use user::UpdateUserURL;
use uuid::Uuid;
use validator::Validate;
use crate::error::AuthError;
use crate::models::user::Token;
use crate::services::auth_service::verify_auth_token;
// Import the hash_password and verify_password functions
use crate::services::hash::{hash_password, verify_password};

// Implement the routers for the server

#[get("/user/{id}")]
async fn get_user(
    username: Path<String>,
    db: Data<Database>,
) -> Result<Json<user::User>, UserError> {
    let username = username.clone();
    let users = Database::get_all_users(&db).await;
    match users {
        Some(all_users) => Ok(Json(
            all_users
                .into_iter()
                .find(|user| user.username == username)
                .ok_or(UserError::NoSuchUser)?,
        )),
        None => Err(UserError::NoUserFound),
    }
}

#[get("/test")]
async fn test_handler() -> impl Responder {
    Json("This is a test!".to_string())
}
// #[get("/test")]
// async fn test_handler() -> impl Responder {
//     Json("This is a test!".to_string())
// }
#[get("/users")]
async fn get_users(db: Data<Database>) -> Result<Json<Vec<user::User>>, UserError> {
    let users = Database::get_all_users(&db).await;
    match users {
        Some(all_users) => Ok(Json(all_users)),
        None => Err(UserError::NoUserFound),
    }
    // HttpResponse::Ok().body("Get all users")
}

#[post("/new_user")]
async fn create_user(
    body: Json<NewUser>,
    db: Data<Database>,
) -> Result<Json<user::User>, UserError> {
    // Create a new user with id and password
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let mut buffer = Uuid::encode_buffer();
            let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);
            let hash_password = hash_password(body.password.clone()).unwrap();

            let new_user = Database::add_user(
                &db,
                user::User::new(
                    new_uuid.to_string(),
                    body.username.clone(),
                    hash_password,
                    false,
                ),
            )
            .await;

            match new_user {
                Ok(create_user) => Ok(Json(create_user)),
                Err(err) => Err(err),
            }
        }
        Err(err) => {
            eprintln!("Error validating user: {:?}", err);
            Err(UserError::UserCreationInvalid)
        },
    }
}

#[post("/update_user/{uuid}")]
async fn update_user(
    update_user_url: Path<UpdateUserURL>,
    db: Data<Database>,
) -> Result<Json<user::User>, UserError> {
    let uuid = update_user_url.uuid.clone();
    let updated_user = Database::update_user(&db, &uuid).await;
    match updated_user {
        Some(user) => Ok(Json(user)),
        None => Err(UserError::NoSuchUser),
    }
}

#[post("/login")]
async fn login_user(body: Json<NewUser>, db: Data<Database>) -> Result<Json<String>, AuthError> {
    let user = Database::get_user(&db, &body.username).await?;
    verify_password(&body.password, &user.password)?;
    match Database::update_user_status(&db, &user.username).await {
        Ok(_) => {
            let token = jwt::generate_token(&user.username)?;
            Ok(Json(token))
        }
        Err(err) => Err(AuthError::from(err)),
    }
}

#[post("/logout")]
async fn logout_user(token: Json<Token>, db: Data<Database>) -> Result<Json<user::User>, AuthError> {
    let username = verify_auth_token(&token.token).await?;
    match Database::update_user_status(&db, &username).await {
        Ok(user) => Ok(Json(user)),
        Err(err) => Err(AuthError::from(err)),
    }
}
