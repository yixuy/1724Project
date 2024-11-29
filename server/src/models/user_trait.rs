use crate::db::Database;
use crate::models::user::User;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

use super::user;

// Implement the UserTrait(function) for the Database struct
#[async_trait]
pub trait UserTrait {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>>;
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User>;
    async fn update_user(db: &Data<Database>, uuid: &str) -> Option<User>;
    async fn update_user_status(db: &Data<Database>, uuid: &str) -> Option<User>;
}

// #[derive(Validate, Debug, Serialize, Deserialize)]
#[async_trait]
impl UserTrait for Database {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>> {
        let users = db.client.select("user").await;
        match users {
            Ok(users) => {
                let users: Vec<User> = users.try_into().unwrap();
                Some(users)
            }
            Err(_) => None,
        }
    }
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User> {
        let user = db
            .client
            .create(("user", new_user.uuid.clone())) // Specify the table and ID
            .content(new_user)
            .await;

        match user {
            Ok(user) => user,
            Err(err) => {
                eprintln!("Error creating user: {:?}", err); // Log the error
                None
            }
        }
    }

    async fn update_user(db: &Data<Database>, uuid: &str) -> Option<User> {
        let user: Result<Option<User>, Error> = db.client.select(("user", uuid)).await;

        match user {
            Ok(this_user) => {
                match this_user {
                    Some(found_user) => {
                        // user.first_name = "John".to_string();
                        // user.last_name = "Doe".to_string();
                        let updated_user = db
                            .client
                            .update(("user", uuid))
                            .merge(User {
                                uuid: found_user.uuid.to_string(),
                                //need to change this to the username
                                username: "John".to_string(),
                                password: found_user.password.clone(),
                                status: found_user.status.clone(),
                            })
                            .await;
                        match updated_user {
                            Ok(updated_user) => updated_user,
                            Err(err) => {
                                eprintln!("Error updating user: {:?}", err); // Log the error
                                None
                            }
                        }
                    }
                    None => None,
                }
            }
            Err(err) => {
                eprintln!("Error updating user: {:?}", err); // Log the error
                None
            }
        }
    }

    async fn update_user_status(db: &Data<Database>, username: &str) -> Option<User> {
        let user: Result<Option<User>, Error> = db.client.select(("user", username)).await;

        match user {
            Ok(this_user) => {
                match this_user {
                    Some(found_user) => {
                        let updated_user = db
                            .client
                            .update(("user", username))
                            .merge(User {
                                uuid: found_user.uuid.to_string(),
                                //need to change this to the username
                                username: found_user.username.to_string(),
                                password: found_user.password.clone(),
                                status: !found_user.status.clone(),
                            })
                            .await;
                        match updated_user {
                            Ok(updated_user) => updated_user,
                            Err(err) => {
                                eprintln!("Error updating user: {:?}", err); // Log the error
                                None
                            }
                        }
                    }
                    None => None,
                }
            }
            Err(err) => {
                eprintln!("Error updating user: {:?}", err); // Log the error
                None
            }
        }
    }
}
