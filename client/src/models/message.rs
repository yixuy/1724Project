use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Properties, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub username: String,
    pub content: String,
    // pub timestamp: String,
}
