use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::collections::{HashMap, HashSet};

// 1. 定义消息类型
#[derive(Message)]
#[rtype(result = "()")]
pub struct TextMessage(pub String);

// Chat Server 是 WebSocket Server
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

//2. 
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

//3. 
#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub id: usize,
    pub room: String,
}

impl Handler<JoinRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) {
        self.rooms
            .entry(msg.room.clone())
            .or_insert_with(HashSet::new)
            .insert(msg.id);
    }
}

// WebSocket Session
pub struct ChatSession {
    id: usize, // 客户端唯一ID
    room: Option<String>, // 当前所在聊天室
    addr: Addr<ChatServer>, // 服务器的地址
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

    //客户端连接时触发，将客户端注册到服务器
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr
            .do_send(Connect {
                id: self.id,
                addr: addr.recipient(),
            });
    }

    //客户端断开时触发，将其从聊天室移除
    fn stopped(&mut self, _: &mut Self::Context) {
        if let Some(room) = &self.room {
            self.addr.do_send(LeaveRoom {
                id: self.id,
                room: room.clone(),
            });
        }
    }
}

//4.
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

impl Handler<TextMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

//handle the command sent from clients
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


// WebSocket 路由
pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    // 通过发送消息生成一个唯一的 ID
    let id = srv.send(GenerateId).await.unwrap_or(0);

    let session = ChatSession::new(id, srv.get_ref().clone());
    ws::start(session, &req, stream)
}