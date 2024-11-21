use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::user::{self, User};

// struct User {
//     first_name: String,
//     last_name: String,
// }
#[derive(Debug, Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:5050").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        client.use_ns("surreal").use_db("chat").await?;

        Ok(Database {
            client,
            name_space: "surreal".to_string(),
            db_name: "chat".to_string(),
        })
    }

    pub async fn get_all_users(&self) -> Option<Vec<User>> {
        let users = self.client.select("user").await;
        match users {
            Ok(users) => {
                let users = users.try_into().unwrap();
                Some(users)
            }
            Err(_) => None,
        }
    }

    pub async fn add_user(&self, new_user: User) -> Option<User> {
        let user = self
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

    pub async fn update_user(&self, uuid: &str) -> Option<User> {
        let user: Result<Option<User>, Error> = self.client.select(("user", uuid)).await;

        match user {
            Ok(this_user) => {
                match this_user {
                    Some(mut found_user) => {
                        // user.first_name = "John".to_string();
                        // user.last_name = "Doe".to_string();
                        let updated_user = self
                            .client
                            .update(("user", uuid))
                            .merge(User {
                                uuid: found_user.uuid.to_string(),
                                //need to change this to the username
                                username: "John".to_string(),
                                password: found_user.password.clone(),
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
