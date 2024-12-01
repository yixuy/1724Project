use crate::models::room::RoomProps;
use yew::prelude::*;

#[function_component(Room)]
pub fn room(props: &RoomProps) -> Html {

    // let user = get_user();
    html! {
        <h1>{ format!("Room: {}", props.room) }</h1>
    }
}


