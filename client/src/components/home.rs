use crate::router::Route;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let room_number = use_state(|| 0);

    let oninput = {
        let room_number = room_number.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                if let Ok(num) = input.value().parse::<i32>() {
                    room_number.set(num);
                }
            }
        })
    };
    let onclick = {
        let room_number = room_number.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Room {
                room: room_number.to_string(),
            });
        })
    };

    html! {
        <div>
            <h1>{ "Home" }</h1>
            <input value={room_number.to_string()} {oninput} />
            <p>{ format!("Room number: {}", *room_number) }</p>
            <button {onclick}>{"Join the room"}</button>
        </div>
    }
}
