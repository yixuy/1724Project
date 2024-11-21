
pub struct Message {
    pub message: String,
    pub sender: String,
    pub receiver: String,
    pub timestamp: String,
}

pub struct Conversation {
    pub sender: String,
    pub receiver: String,
    pub messages: Vec<Message>,
}