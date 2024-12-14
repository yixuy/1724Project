use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::collections::{HashMap, HashSet};

// 1. Define
#[derive(Message)]
#[rtype(result = "()")]
pub struct TextMessage(pub String);

// Chat Server
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<TextMessage>>,
    rooms: HashMap<String, HashSet<usize>>, //room -> client_id
    id_counter: usize, //client_id
}

impl ChatServer {
    pub fn new() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            id_counter: 0,
        }
    }

    fn send_message(&self, room: &str, message: &str) { //broadcast message to all clients in the room
        if let Some(participants) = self.rooms.get(room) {
            for id in participants {
                if let Some(recipient) = self.sessions.get(id) {
                    let _ = recipient.do_send(TextMessage(message.to_owned()));
                }
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

//2. Connect
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: usize,
    pub addr: Recipient<TextMessage>,
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id, msg.addr);
    }
}

//3.1 CRATE ROOM消息
#[derive(Message)]
#[rtype(result = "Result<(), String>")] 
pub struct CreateRoom {
    pub id: usize,
    pub room: String,
}

impl Handler<CreateRoom> for ChatServer {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: CreateRoom, _: &mut Context<Self>) -> Self::Result {
        if self.rooms.contains_key(&msg.room) {
            Err(format!("Room '{}' already exists!", msg.room))
        } else {
            self.rooms.insert(msg.room.clone(), HashSet::new());
            self.rooms.get_mut(&msg.room).unwrap().insert(msg.id);
            Ok(())
        }
    }
}


//3.2 JOIN ROOM消息
#[derive(Message)]
#[rtype(result = "Result<(), String>")]
pub struct JoinRoom {
    pub id: usize,
    pub room: String,
}
/** 修改原逻辑
impl Handler<JoinRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) {
        self.rooms
            .entry(msg.room.clone())
            .or_insert_with(HashSet::new)
            .insert(msg.id);
    }
}
**/

impl Handler<JoinRoom> for ChatServer {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) -> Self::Result {
        if let Some(participants) = self.rooms.get_mut(&msg.room) {
            participants.insert(msg.id);
            Ok(())
        } else {
            Err(format!("Room '{}' does not exist!", msg.room))
        }
    }
}


//3.3 LEAVE ROOM消息
#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveRoom {
    pub id: usize,
    pub room: String,
}

// if a room is empty, remove it
impl Handler<LeaveRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: LeaveRoom, _: &mut Context<Self>) {
        if let Some(participants) = self.rooms.get_mut(&msg.room) {
            participants.remove(&msg.id);
            if participants.is_empty() {
                self.rooms.remove(&msg.room);
            }
        }
    }
}



// 4. WebSocket Session
pub struct ChatSession {
    id: usize, //Client ID
    room: Option<String>, 
    addr: Addr<ChatServer>, 
}

impl ChatSession {
    pub fn new(id: usize, addr: Addr<ChatServer>) -> Self {
        ChatSession {
            id,
            room: None,
            addr,
        }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr
            .do_send(Connect {
                id: self.id,
                addr: addr.recipient(),
            });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        if let Some(room) = &self.room {
            self.addr.do_send(LeaveRoom {
                id: self.id,
                room: room.clone(),
            });
        }
    }
}

///////////////////////////////////////////

/** 
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            let text = text.to_string();
            
            if text.starts_with("/join ") {
                let room = text.trim_start_matches("/join ").to_owned();
                self.room = Some(room.clone());
                self.addr.do_send(JoinRoom {
                    id: self.id,
                    room,
                });
            } else if let Some(room) = &self.room {
                self.addr.do_send(BroadcastMessage {
                    room_name: room.clone(),
                    message: text,
                });
            }
        }
    }
}
**/

//streamhandler修改
//SteamHandler是Actix的一个特性，表示该结构体可以处理流式事件
//ctx 是 Actix 提供的上下文对象，用于控制会话状态或发送响应

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            let trimmed_text = text.trim();

            if trimmed_text.starts_with("/create ") {
                let room = trimmed_text.trim_start_matches("/create ").to_owned();
                let addr = self.addr.clone();
                let id = self.id;

                ctx.spawn(
                    async move {
                        let result = addr.send(CreateRoom { id, room: room.clone() }).await.unwrap_or(Err("Error".into()));
                        (result, room)
                    }
                    .into_actor(self)
                    .map(|(result, room), _act, ctx| {
                        match result {
                            Ok(_) => ctx.text(format!("Room '{}' created successfully. ", room)),
                            Err(err) if err == "Room already exists" => {
                                ctx.text(format!("Room '{}' already exists! Type '/join {}' to join it.", room, room));
                            }
                            Err(err) => ctx.text(format!("Failed to create room: {}", err)),
                        }
                    }),
                );

            } else if trimmed_text.starts_with("/join ") {
                let room = trimmed_text.trim_start_matches("/join ").to_owned();
                let addr = self.addr.clone();
                let id = self.id;

                if room.is_empty() {
                    ctx.text("Invalid command. Usage: '/join <room_name>'");
                    return;
                }

                ctx.spawn(
                    async move {
                        let result = addr.send(JoinRoom { id, room: room.clone() }).await.unwrap_or(Err("Error".into()));
                        (result, room)
                    }
                    .into_actor(self)
                    .map(|(result, room), act, ctx| {
                        match result {
                            Ok(_) => {
                                act.room = Some(room.clone());
                                ctx.text(format!("Successfully joined the room '{}'. ", room));
                                ctx.text(format!("You can now start chatting!"));
                            }
                            Err(err) => ctx.text(format!("Failed to join room: {}", err)),
                        }
                    }),
                );

            } else if let Some(room) = &self.room {
                
                self.addr.do_send(BroadcastMessage {
                    room_name: room.clone(),
                    message: trimmed_text.to_owned(),
                });
            } else {
                
                ctx.text("You must join a room first. Use '/join <room_name>' to join an existing room.");
            }

        } else if let Ok(ws::Message::Close(_)) = msg {           
            ctx.stop();
        } else if let Ok(ws::Message::Ping(msg)) = msg {
            ctx.pong(&msg);
        }
    }
}



//5.
#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub room_name: String,
    pub message: String,
}

impl Handler<BroadcastMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) {
        self.send_message(&msg.room_name, &msg.message);
    }
}

impl Handler<TextMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

//6.
#[derive(Message)]
#[rtype(result = "usize")] 
pub struct GenerateId;

impl Handler<GenerateId> for ChatServer {
    type Result = usize;

    fn handle(&mut self, _: GenerateId, _: &mut Context<Self>) -> Self::Result {
        self.id_counter += 1;
        self.id_counter
    }
}


// WebSocket Router
pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    // Generate a Unique ID
    let id = srv.send(GenerateId).await.unwrap_or(0);

    let session = ChatSession::new(id, srv.get_ref().clone());
    ws::start(session, &req, stream)
}
