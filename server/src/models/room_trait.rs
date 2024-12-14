use super::message::ChatMessage;
use crate::db::Database;
use crate::models::room::*;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

// Implement the UserTrait(function) for the Database struct
#[async_trait]
pub trait RoomTrait {
    async fn create_new_room(db: &Data<Database>, new_room: Room) -> Option<Room>;
    async fn get_all_rooms(db: &Data<Database>) -> Option<Vec<Room>>;
    async fn get_messages_from_room(db: &Data<Database>, room_id: &str)
        -> Option<Vec<ChatMessage>>;
    async fn update_room_user(db: &Data<Database>, room_id: String, user: String) -> Option<Room>;
    async fn update_messages_from_room(
        db: &Data<Database>,
        room_id: String,
        message: ChatMessage,
    ) -> Option<Room>;
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
                eprintln!("Error creating room: {:?}", err); // Log the error
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

    async fn get_messages_from_room(
        db: &Data<Database>,
        room_id: &str,
    ) -> Option<Vec<ChatMessage>> {
        let rooms = db.client.select("room").await;
        match rooms {
            Ok(rooms) => {
                let mut cloned_rooms = rooms.clone();
                let room = cloned_rooms
                    .iter_mut()
                    .find(|room: &&mut Room| room.room_id == room_id);

                Some(room.unwrap().messages.clone())
            }
            Err(_) => None,
        }
    }

    async fn update_room_user(db: &Data<Database>, room_id: String, user: String) -> Option<Room> {
        let room_id_clone = room_id.clone();
        let room: Result<Option<Room>, Error> = db.client.select(("room", room_id_clone)).await;
        match room {
            Ok(Some(room)) => {
                let mut users = room.users.clone();
                if users.contains(&user) {
                    return None;
                }
                users.push(user);
                let updated_room = db
                    .client
                    .update(("room", room_id.clone()))
                    .merge(Room {
                        room_id: room.room_id.to_string(),
                        users,
                        messages: room.messages.clone(),
                    })
                    .await;

                match updated_room {
                    Ok(updated_room) => updated_room,
                    Err(err) => {
                        eprintln!("Error updating user status: {:?}", err);
                        None
                    }
                }
            }
            Ok(None) => {
                eprintln!("Room not found");
                None
            }
            Err(err) => {
                eprintln!("Error fetching room: {:?}", err);
                None
            }
        }
    }

    async fn update_messages_from_room(
        db: &Data<Database>,
        room_id: String,
        message: ChatMessage,
    ) -> Option<Room> {
        let room_id_clone = room_id.clone();
        let room: Result<Option<Room>, Error> = db.client.select(("room", room_id_clone)).await;
        match room {
            Ok(Some(room)) => {
                let mut messages = room.messages.clone();
                messages.push(message);
                let updated_room = db
                    .client
                    .update(("room", room_id))
                    .merge(Room {
                        room_id: room.room_id.to_string(),
                        users: room.users.clone(),
                        messages: messages,
                    })
                    .await;

                match updated_room {
                    Ok(updated_room) => updated_room,
                    Err(err) => {
                        eprintln!("Error updating user status: {:?}", err);
                        None
                    }
                }
            }
            Ok(None) => {
                eprintln!("Room not found");
                None
            }
            Err(err) => {
                eprintln!("Error fetching room: {:?}", err);
                None
            }
        }
    }
}
