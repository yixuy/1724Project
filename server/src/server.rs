use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use actix::prelude::*;
use actix::{Actor, Addr, Context, Handler, Message, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub username: String,
    pub content: String,
}

// ChatServer manages chat rooms
pub struct ChatServer {
    rooms: HashMap<String, Vec<Addr<WebSocketSession>>>,
    histories: HashMap<String, Vec<ChatMessage>>,
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            rooms: HashMap::new(),
            histories: HashMap::new(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

// Handle Join
impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let room_users = self.rooms.entry(msg.room_id.clone()).or_default();
        room_users.push(msg.addr);
        for user in room_users {
            if let Some(history) = self.histories.get(&msg.room_id) {
                let history_json = serde_json::to_string(history).unwrap();
                user.do_send(ClientMessage(history_json));
            }
        }
        println!("User '{}' joined room '{}'", msg.username, msg.room_id);
    }
}

// Handle Leave
impl Handler<Leave> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Leave, _: &mut Context<Self>) {
        if let Some(users) = self.rooms.get_mut(&msg.room_id) {
            users.retain(|addr| addr.connected());
            println!("User '{}' left room '{}'", msg.username, msg.room_id);
        }
    }
}

impl Handler<MessageToRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: MessageToRoom, _: &mut Context<Self>) {
        if let Some(users) = self.rooms.get(&msg.room_id) {
            println!("aaa{}", msg.message);
            let chat_message: ChatMessage = serde_json::from_str(&msg.message).unwrap();

            self.histories
                .entry(msg.room_id.clone())
                .or_insert_with(Vec::new)
                .push(chat_message);
            // println!("history{:?}", self.histories.get(&msg.room_id));
            for user in users {
                if let Some(history) = self.histories.get(&msg.room_id) {
                    let history_json = serde_json::to_string(history).unwrap();
                    user.do_send(ClientMessage(history_json));
                }
            }
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
        println!("message: {}", msg.0);
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
) -> Result<HttpResponse, Error> {
    let (username, room_id) = path.into_inner();
    let session = WebSocketSession {
        username,
        room_id,
        server_addr: server_addr.get_ref().clone(),
    };
    ws::start(session, &req, stream)
}
