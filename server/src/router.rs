use crate::db;
use crate::error;
use crate::user;
use actix_web::web::Data;
use actix_web::{get, post, web::Json, web::Path, HttpResponse, Responder};
use db::Database;
use error::UserError;
use user::NewUser;
use user::UpdateUserURL;
use uuid::Uuid;
use validator::Validate;

#[get("/user/{id}")]
async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("User")
}

#[get("/test")]
async fn test_handler() -> impl Responder {
    HttpResponse::Ok().body("This is a test!")
}

#[get("/users")]
async fn get_users(db: Data<Database>) -> Result<Json<Vec<user::User>>, UserError> {
    let users = db.get_all_users().await;
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

            let new_user = db
                .add_user(user::User::new(
                    new_uuid.to_string(),
                    body.username.clone(),
                    body.password.clone(),
                ))
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
    let updated_user = db.update_user(&uuid).await;
    match updated_user {
        Some(user) => Ok(Json(user)),
        None => Err(UserError::NoSuchUser),
    }
}
