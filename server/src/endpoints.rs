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
                Some(create_user) => Ok(Json(create_user)),
                None => Err(UserError::UserCreationFailed),
            }
        }
        Err(_e) => Err(UserError::UserCreationFailed),
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
async fn login_user(body: Json<NewUser>, db: Data<Database>) -> Result<Json<String>, UserError> {
    let users = Database::get_all_users(&db).await;
    match users {
        Some(all_users) => {
            let user = all_users
                .into_iter()
                .find(|user| {
                    user.username == body.username
                        && verify_password(&body.password, &user.password).unwrap_or(false)
                })
                .ok_or(UserError::NoSuchUser)?;
            // user.status = true;
            match Database::update_user_status(&db, &user.uuid).await {
                Some(_) => {
                    let token =
                        jwt::generate_token(&user.username).map_err(|_| UserError::NoSuchUser)?;
                    Ok(Json(token))
                }
                None => Err(UserError::UserUpdateFailed),
            }
        }
        None => Err(UserError::NoUserFound),
    }
}

// pub async fn logout_user(token: &str) -> Result<(), AuthError> {
//     // validate the token and get the username
//     let username = verify_token(token)?;

//     // update the user's status to offline
//     status_service::set_offline(&username)
//         .await
//         .map_err(|err| {
//             eprintln!("Error updating user status to offline: {:?}", err);
//             AuthError::StatusUpdateError
//         })?;

//     Ok(())
// }
