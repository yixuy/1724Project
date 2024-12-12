use crate::db::Database;
use crate::error::UserError;
use crate::models::user::User;
use crate::models::user::UserStatus;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

// Implement the UserTrait(function) for the Database struct
#[async_trait]
pub trait UserTrait {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>>;
    async fn get_user(db: &Data<Database>, user: &str) -> Result<User, UserError>;
    async fn add_user(db: &Data<Database>, new_user: User) -> Result<User, UserError>;
    async fn get_user_status(db: &Data<Database>, username: &str) -> Result<UserStatus, UserError>;

    async fn update_user(db: &Data<Database>, uuid: &str) -> Option<User>;
    async fn update_user_status(
        db: &Data<Database>,
        username: &str,
        status: UserStatus,
    ) -> Result<User, UserError>;
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
    async fn get_user(db: &Data<Database>, username: &str) -> Result<User, UserError> {
        let user: Result<Option<User>, Error> = db.client.select(("user", username)).await;

        match user {
            Ok(Some(found_user)) => Ok(found_user),
            Ok(None) => Err(UserError::NoSuchUser),
            Err(err) => {
                eprintln!("Error getting user: {:?}", err); // Log the error
                Err(UserError::UserSearchFailed)
            }
        }
    }

    async fn add_user(db: &Data<Database>, new_user: User) -> Result<User, UserError> {
        let existing_user: Result<Option<User>, Error> =
            db.client.select(("user", &new_user.username)).await;

        match existing_user {
            Ok(Some(_)) => Err(UserError::UsernameExists),
            Ok(None) => {
                let user = db
                    .client
                    .create(("user", new_user.username.clone()))
                    .content(new_user)
                    .await;

                match user {
                    Ok(user) => Ok(user.unwrap()),
                    Err(err) => {
                        eprintln!("Error creating user: {:?}", err); // Log the error
                        Err(UserError::UserCreationFailed)
                    }
                }
            }
            Err(err) => {
                eprintln!("Error creating user: {:?}", err); // Log the error
                Err(UserError::UserCreationFailed)
            }
        }
    }
    async fn get_user_status(db: &Data<Database>, username: &str) -> Result<UserStatus, UserError> {
        let user: Result<Option<User>, Error> = db.client.select(("user", username)).await;

        match user {
            Ok(Some(user)) => Ok(user.status),
            Ok(None) => Err(UserError::NoSuchUser),
            Err(err) => {
                eprintln!("Error getting user status: {:?}", err); // Log the error
                Err(UserError::UserSearchFailed)
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
                        updated_user.unwrap_or_else(|err| {
                            eprintln!("Error updating user: {:?}", err); // Log the error
                            None
                        })
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

    async fn update_user_status(
        db: &Data<Database>,
        username: &str,
        status: UserStatus,
    ) -> Result<User, UserError> {
        let user: Result<Option<User>, Error> = db.client.select(("user", username)).await;
        match user {
            Ok(Some(user)) => {
                let updated_user = db
                    .client
                    .update(("user", username))
                    .merge(User {
                        uuid: user.uuid,
                        username: user.username,
                        password: user.password,
                        status: status,
                    })
                    .await;

                match updated_user {
                    Ok(updated_user) => Ok(updated_user.unwrap()),
                    Err(err) => {
                        eprintln!("Error updating user status: {:?}", err);
                        Err(UserError::UserUpdateFailed)
                    }
                }
            }
            Ok(None) => Err(UserError::NoSuchUser),
            Err(err) => {
                eprintln!("Error updating user status: {:?}", err);
                Err(UserError::UserUpdateFailed)
            }
        }
    }
}
