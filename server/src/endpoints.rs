use crate::db;
use crate::error::AuthError;
use crate::error::RoomError;
use crate::error::TokenError;
use crate::error::UserError;
use crate::models::room::NewRoom;
use crate::models::room::Room;
use crate::models::room_trait::RoomTrait;
use crate::models::user;
use crate::models::user_trait::UserTrait;
use crate::services::hash::{hash_password, verify_password};
use crate::services::prelude::*;
use actix_web::web::Data;
use actix_web::{get, post, web::Json, web::Path, Responder};
use db::Database;
use user::NewUser;
use user::UpdateUserURL;
use uuid::Uuid;
use validator::Validate;

// Implement the routers for the server

#[get("/user/{token}")]
async fn get_user(token: Path<String>, db: Data<Database>) -> Result<Json<user::User>, AuthError> {
    let token = token.clone();
    let username = jwt::verify_token(&token)
        .await
        .map_err(|_| TokenError::TokenInvalid)?
        .sub;
    let users = Database::get_all_users(&db).await;
    match users {
        Some(all_users) => Ok(Json(
            all_users
                .into_iter()
                .find(|user| user.username == username)
                .ok_or(UserError::NoSuchUser)?,
        )),
        None => Err(UserError::NoUserFound.into()),
    }
}

#[get("/test")]
async fn test_handler() -> impl Responder {
    Json("This is a test!".to_string())
}

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
                    user::UserStatus::Offline,
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
        }
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
    match Database::update_user_status(&db, &user.username, user::UserStatus::Online).await {
        Ok(_) => {
            let token = jwt::generate_token(&user.username)?;
            Ok(Json(token))
        }
        Err(err) => Err(AuthError::from(err)),
    }
}

#[get("/get_status/{username}")]
async fn get_user_status(
    username: Path<String>,
    db: Data<Database>,
) -> Result<Json<user::UserStatus>, UserError> {
    let username = username.clone();
    let user = Database::get_user(&db, &username).await?;
    Ok(Json(user.status.clone()))
}

#[post("/logout/{username}")]
pub async fn logout_user(
    username: Path<String>,
    db: Data<Database>,
) -> Result<Json<String>, AuthError> {
    // update the user's status to offline
    Database::set_offline(&db, &username.into_inner())
        .await
        .map_err(|err| {
            eprintln!("Error updating user status to offline: {:?}", err);
            AuthError::from(err)
        })?;
    Ok(Json("User logged out successfully".to_string()))
}

#[post("/create_room")]
async fn create_room(body: Json<NewRoom>, db: Data<Database>) -> Result<Json<Room>, RoomError> {
    let new_room = Database::create_new_room(&db, Room::new(body.room_id.clone())).await;

    match new_room {
        Some(new_room) => Ok(Json(new_room)),
        None => Err(RoomError::RoomCreationFailed),
    }
}

#[get("/rooms")]
async fn get_rooms(db: Data<Database>) -> Result<Json<Vec<Room>>, RoomError> {
    let rooms = Database::get_all_rooms(&db).await;
    match rooms {
        Some(all_rooms) => Ok(Json(all_rooms)),
        None => Err(RoomError::NoRoomFound),
    }
    // HttpResponse::Ok().body("Get all users")
}

#[get("/room/{room_id}")]
async fn get_room(room_id: Path<String>, db: Data<Database>) -> Result<Json<Room>, AuthError> {
    let room_id = room_id.clone();
    let rooms = Database::get_all_rooms(&db).await;
    // println!("Rooms: {:?}", rooms);
    match rooms {
        Some(all_rooms) => Ok(Json(
            all_rooms
                .into_iter()
                .find(|room| room.room_id == room_id)
                .ok_or(RoomError::NoSuchRoom)?,
        )),
        None => Err(RoomError::NoRoomFound.into()),
    }
}
