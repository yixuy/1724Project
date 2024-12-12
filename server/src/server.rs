use crate::db::Database;
use crate::models::prelude::*;
use crate::models::user::UserStatus;
use crate::models::user_trait::UserTrait;
use actix::prelude::*;
use actix::{Actor, Addr, Context, Handler, Message, StreamHandler};
use actix_web::web::Data;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use chrono::Local;
use std::collections::HashMap;

// Define messages
#[derive(Message)]
#[rtype(result = "()")]
struct Join {
    username: String,
    room_id: String,
    addr: Addr<WebSocketSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Leave {
    username: String,
    room_id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
struct MessageToRoom {
    room_id: String,
    message: String,
}

#[derive(Debug, Clone)]
pub struct ChatServer {
    rooms: HashMap<String, Vec<Addr<WebSocketSession>>>,
    db: Data<Database>,
}

impl ChatServer {
    pub fn new(db: Data<Database>) -> Self {
        ChatServer {
            rooms: HashMap::new(),
            db,
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("ChatServer started");
    }
}

// Handle Join
impl Handler<Join> for ChatServer {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) -> Self::Result {
        let room_users = self.rooms.entry(msg.room_id.clone()).or_default();
        room_users.push(msg.addr);
        let db_clone = self.db.clone();
        let room_id = msg.room_id.clone();
        let username = msg.username.clone();

        let room_users_clone = room_users.clone();

        let fut = async move {
            let update_room =
                Database::update_room_user(&db_clone, room_id.clone(), username.clone()).await;
            Database::update_user_status(&db_clone, &username, UserStatus::Online)
                .await
                .unwrap();
            let room_msg;
            if update_room.is_some() {
                room_msg = update_room.unwrap().messages.clone();
            } else {
                room_msg = Database::get_messages_from_room(&db_clone, &room_id)
                    .await
                    .unwrap_or_default();
            }
            // println!("Room messages: {:?}", room_msg);
            for user in room_users_clone {
                let history_json = serde_json::to_string(&room_msg).unwrap();
                user.do_send(ClientMessage(history_json));
            }
            println!("User '{}' joined room '{}'", username, room_id);
        };
        Box::pin(fut.into_actor(self).map(|_, _, _| ()))
    }
}

// Handle Leave
impl Handler<Leave> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Leave, _: &mut Context<Self>) {
        let db_clone = self.db.clone();

        let username = msg.username.clone();
        if let Some(users) = self.rooms.get_mut(&msg.room_id) {
            users.retain(|addr| addr.connected());
            let db_clone = db_clone.clone();
            let username = username.clone();
            actix::spawn(async move {
                Database::update_user_status(&db_clone, &username, UserStatus::Leave)
                    .await
                    .unwrap();
            });
            println!("User '{}' left room '{}'", msg.username, msg.room_id);
        }
    }
}

impl Handler<MessageToRoom> for ChatServer {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: MessageToRoom, _: &mut Context<Self>) -> Self::Result {
        if let Some(users) = self.rooms.get(&msg.room_id) {
            // println!("aaa{}", msg.message);
            let local_time = Local::now().time().format("%H:%M:%S").to_string();
            let local_date = Local::now().date().format("%Y-%m-%d").to_string();

            let chat_message_json: NewChatMessage = serde_json::from_str(&msg.message).unwrap();

            let chat_message = ChatMessage {
                username: chat_message_json.username.to_string(),
                content: chat_message_json.content.clone(),
                timestamp: local_date + " " + &local_time,
                // user_status: UserStatus::Online,
            };
            let db_clone = self.db.clone();
            let room_id = msg.room_id.clone();
            let chat_message_clone = chat_message.clone();
            let users_clone = users.clone();
            // print!("{:?}", users_clone);
            let fut = async move {
                let update_room = Database::update_messages_from_room(
                    &db_clone,
                    room_id.to_string(),
                    chat_message_clone,
                )
                .await;
                if update_room.is_some() {
                    let room_msg: Vec<ChatMessage> = update_room.unwrap().messages.clone();

                    for user in users_clone {
                        let history_json = serde_json::to_string(&room_msg).unwrap();
                        user.do_send(ClientMessage(history_json));
                    }
                }
                println!("Send message to room '{}'", room_id);
            };

            Box::pin(fut.into_actor(self).map(|_, _, _| ()))
        } else {
            Box::pin(async {}.into_actor(self).map(|_, _, _| ()))
        }
    }
}

// Define a message to send to clients
#[derive(Message)]
#[rtype(result = "()")]
struct ClientMessage(String);

impl Handler<ClientMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

pub struct WebSocketSession {
    pub username: String,
    pub room_id: String,
    pub server_addr: Addr<ChatServer>,
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Notify server about joining the room
        self.server_addr.do_send(Join {
            username: self.username.clone(),
            room_id: self.room_id.clone(),
            addr: ctx.address(),
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        // Notify server about leaving the room
        self.server_addr.do_send(Leave {
            username: self.username.clone(),
            room_id: self.room_id.clone(),
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Broadcast the received message to the room
                let formatted_message = format!("{}", text);
                self.server_addr.do_send(MessageToRoom {
                    room_id: self.room_id.clone(),
                    message: formatted_message,
                });
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => {}
            Ok(ws::Message::Binary(_)) => {
                ctx.text("Binary messages are not supported.");
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Err(e) => {
                println!("WebSocket error: {:?}", e);
                ctx.stop();
            }
            _ => {}
        }
    }
}

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<(String, String)>,
    server_addr: web::Data<Addr<ChatServer>>,
    // db: Data<Database>,
) -> Result<HttpResponse, Error> {
    let (username, room_id) = path.into_inner();
    let session = WebSocketSession {
        username,
        room_id,
        server_addr: server_addr.get_ref().clone(),
    };
    ws::start(session, &req, stream)
}
