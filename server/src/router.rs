use crate::db;
use crate::error;
// use crate::user;
// use crate::user_trait::UserTrait;
use crate::models::user;
use crate::models::user_trait::UserTrait;
use actix_web::web::Data;
use actix_web::{get, post, web::Json, web::Path, HttpResponse, Responder};
use db::Database;
use error::UserError;
use user::NewUser;
use user::UpdateUserURL;
use uuid::Uuid;
use validator::Validate;

// Implement the routers for the server

#[get("/user/{id}")]
async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("User")
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

            let new_user = Database::add_user(
                &db,
                user::User::new(
                    new_uuid.to_string(),
                    body.username.clone(),
                    body.password.clone(),
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
