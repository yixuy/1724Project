use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RoomProps {
    pub room: String,
}

#[function_component(Room)]
pub fn room(props: &RoomProps) -> Html {
    html! {
        <h1>{ format!("Room: {}", props.room) }</h1>
    }
}