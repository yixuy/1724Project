use crate::error::DatabaseError;
use crate::models::user::User;

/* Database operations for user data */

pub async fn create_user(username: &str, hashed_password: &str) -> Result<(), DatabaseError> {
    todo!();
    // check if the username already exists
    // if not, inserts a new user into the database
    // query should insert a user using uuid, username, hashed_password, status, etc.
    // Ok(()) or Err(UsernameExists)
}

// pub async fn get_user_by_username(username: &str) -> Result<User, DatabaseError> {
//     todo!();
//     // fetches a user by username
//     // query should get the user info for this username
//     // let user;
//     // user = user.from_db_model();
//     // Ok(user) or Err(UserNotFound)
// }

pub async fn update_user_status(username: &str, is_online: bool) -> Result<(), DatabaseError> {
    todo!();
    // updates a user's online/offline status
    // let status = if is_online { "online" } else { "offline" };
    // query should update and set the status value for this username
    // Ok(())
}

pub async fn get_user_status(username: &str) -> Result<String, DatabaseError> {
    todo!();
    // fetch a user's current status
    // query should get the status for this username
    // Ok("online".to_string()) or Err(UserNotFound)
}
