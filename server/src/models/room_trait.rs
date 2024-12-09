use super::message::{self, Message};
use crate::db::Database;
use crate::models::room::*;
use crate::models::user::User;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

// Implement the UserTrait(function) for the Database struct
#[async_trait]
pub trait RoomTrait {
    async fn create_new_room(db: &Data<Database>, new_room: Room) -> Option<Room>;
    async fn get_all_rooms(db: &Data<Database>) -> Option<Vec<Room>>;
    async fn update_room_user(
        db: &Data<Database>,
        room_id: &str,
        user: User,
    ) -> Result<Vec<Room>, Error>;

    // async fn update_room_message(
    //     db: &Data<Database>,
    //     room_id: &str,
    //     message: Message,
    // ) -> Result<Vec<Room>, Error>;
    // async fn update_user_status(db: &Data<Database>, uuid: &str) -> Option<User>;
}

// #[derive(Validate, Debug, Serialize, Deserialize)]
#[async_trait]
impl RoomTrait for Database {
    async fn create_new_room(db: &Data<Database>, new_room: Room) -> Option<Room> {
        let room = db
            .client
            .create(("room", new_room.room_id.clone())) // Specify the table and ID
            .content(new_room)
            .await;

        match room {
            Ok(room) => room,
            Err(err) => {
                eprintln!("Error creating user: {:?}", err); // Log the error
                None
            }
        }
    }

    async fn get_all_rooms(db: &Data<Database>) -> Option<Vec<Room>> {
        let rooms = db.client.select("room").await;
        match rooms {
            Ok(rooms) => {
                let rooms: Vec<Room> = rooms.try_into().unwrap();
                Some(rooms)
            }
            Err(_) => None,
        }
    }

    async fn update_room_user(
        db: &Data<Database>,
        room_id: &str,
        user: User,
    ) -> Result<Vec<Room>, Error> {
        let rooms = db.client.select("room").await;

        match rooms {
            Ok(rooms) => {
                let mut rooms: Vec<Room> = rooms.try_into().unwrap();
                let room = rooms
                    .iter_mut()
                    .find(|room| room.room_id == room_id)
                    .unwrap();
                room.users.push(user);
                db.client.update("room").content(rooms).await
            }
            Err(err) => Err(err),
        }
    }

    // async fn update_room_message(
    //     db: &Data<Database>,
    //     room_id: &str,
    //     message: Message,
    // ) -> Result<Vec<Room>, Error> {
    //     let rooms = db.client.select("room").await;

    //     match rooms {
    //         Ok(rooms) => {
    //             let mut rooms: Vec<Room> = rooms.try_into().unwrap();
    //             let room = rooms
    //                 .iter_mut()
    //                 .find(|room| room.room_id == room_id)
    //                 .unwrap();
    //             room.messages.push(message);
    //             db.client.update("room").content(rooms).await
    //         }
    //         Err(err) => Err(err),
    //     }
    // }

    // async fn update_room_message(db: &Data<Database>, uuid: &str) -> Option<Message> {

    // }
    // async fn add_user(db: &Data<Database>, new_user: User) -> Option<User> {
    //     let user = db
    //         .client
    //         .create(("user", new_user.uuid.clone())) // Specify the table and ID
    //         .content(new_user)
    //         .await;

    //     match user {
    //         Ok(user) => user,
    //         Err(err) => {
    //             eprintln!("Error creating user: {:?}", err); // Log the error
    //             None
    //         }
    //     }
    // }

    // async fn update_user(db: &Data<Database>, uuid: &str) -> Option<User> {
    //     let user: Result<Option<User>, Error> = db.client.select(("user", uuid)).await;

    //     match user {
    //         Ok(this_user) => {
    //             match this_user {
    //                 Some(found_user) => {
    //                     // user.first_name = "John".to_string();
    //                     // user.last_name = "Doe".to_string();
    //                     let updated_user = db
    //                         .client
    //                         .update(("user", uuid))
    //                         .merge(User {
    //                             uuid: found_user.uuid.to_string(),
    //                             //need to change this to the username
    //                             username: "John".to_string(),
    //                             password: found_user.password.clone(),
    //                             status: found_user.status.clone(),
    //                         })
    //                         .await;
    //                     match updated_user {
    //                         Ok(updated_user) => updated_user,
    //                         Err(err) => {
    //                             eprintln!("Error updating user: {:?}", err); // Log the error
    //                             None
    //                         }
    //                     }
    //                 }
    //                 None => None,
    //             }
    //         }
    //         Err(err) => {
    //             eprintln!("Error updating user: {:?}", err); // Log the error
    //             None
    //         }
    //     }
    // }

    // async fn update_user_status(db: &Data<Database>, username: &str) -> Option<User> {
    //     let user: Result<Option<User>, Error> = db.client.select(("user", username)).await;

    //     match user {
    //         Ok(this_user) => {
    //             match this_user {
    //                 Some(found_user) => {
    //                     let updated_user = db
    //                         .client
    //                         .update(("user", username))
    //                         .merge(User {
    //                             uuid: found_user.uuid.to_string(),
    //                             //need to change this to the username
    //                             username: found_user.username.to_string(),
    //                             password: found_user.password.clone(),
    //                             status: !found_user.status.clone(),
    //                         })
    //                         .await;
    //                     match updated_user {
    //                         Ok(updated_user) => updated_user,
    //                         Err(err) => {
    //                             eprintln!("Error updating user: {:?}", err); // Log the error
    //                             None
    //                         }
    //                     }
    //                 }
    //                 None => None,
    //             }
    //         }
    //         Err(err) => {
    //             eprintln!("Error updating user: {:?}", err); // Log the error
    //             None
    //         }
    //     }
    // }
}
