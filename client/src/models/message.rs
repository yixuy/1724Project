
use yew::Properties;

#[derive(Properties, PartialEq, Clone)]
pub struct Message {
    pub username: String,
    pub content: String,
    pub timestamp: String,
}
