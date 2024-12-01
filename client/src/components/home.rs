use crate::endpoints::{get_current_user, get_user};
use crate::models::user::User;
use crate::router::Route;
use stylist::style;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
#[function_component(Home)]
pub fn home() -> Html {
    let username = get_current_user().unwrap_or_else(|| "".to_string());
    let user = User::new(username.clone(), "".to_string());
    let fetched = use_state(|| false);
    let user_string = use_state(|| "".to_string());
    if username != "" {
        let user = User::new(username.clone(), "".to_string());
        {
            let mut user = user.clone();
            let user_string_clone = user_string.clone();
            let fetched = fetched.clone();
            if *fetched == false {
                wasm_bindgen_futures::spawn_local(async move {
                    get_user(user_string_clone.clone()).await;
                    let user_json: User = serde_json::from_str(&*user_string_clone)
                        .unwrap_or(User::new("".to_string(), "".to_string()));
                    user.set_username(user_json.username);
                    // user.set_username(user_json);
                    fetched.set(true);
                });
            }
        }
    }
    // let user_json = serde_json::from_str::<User>(&*user_string).unwrap();

    let css = style!(
        r#"
        .container {
            background-color: #4287f5; /* Keeps your background color */
            height: 100vh; /* Ensures full height of the viewport */
            display: flex; /* Enables Flexbox */
            justify-content: center; /* Centers horizontally */
            align-items: center; /* Centers vertically */
            flex-direction: column; /* Stacks children vertically */
            text-align: center; /* Centers text inside the container */
        }
       .card {
        background-color: white;  /* Card's background color */
        border-radius: 20px; /* Rounded corners */
        padding: 20px; /* Padding inside the card */
        height: 150%; /* Set height for the card */
        width: 100%; /* Relative width to adjust with the window size */
        max-height: 500px; /* Maximum height to prevent the card from getting too large */
        min-height: 300px; /* Minimum height to prevent the card from getting too small */
        max-width: 800px; /* Maximum width to prevent the card from getting too large */
        min-width: 300px; /* Minimum width to prevent the card from getting too small */
        margin: 50px auto; /* Center the card horizontally and add top/bottom margin */
        text-align: center; /* Center text inside the card */
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); /* Optional: Adds shadow for a card effect */
    }
        "#
    )
    .unwrap();

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
         <div class={css.get_class_name().to_string()}>
         <div class="container">
            <div class="card">
                // <h1>{ "Please login the application" }</h1>
                // if let Some(room) = room_number.get() {
                //     <h2>{ format!("Room number: {}", room) }</h2>
                // }
                // <h2>{ format!("Welcome, {}!", user.username) }</h2>
                if user.username != ""{
                    <h2>{ format!("Welcome, {}!",user.username) }</h2>
                } else {
                    <h2>{ "Please Sign up the username before you can join the room" }</h2>
                }
                // <h2>{ "Please Sign up the username before you can join the room" }</h2>
                <input value={room_number.to_string()} {oninput} />
                <p>{ format!("Room number: {}", *room_number) }</p>
                // <p>{user_token.as_deref().unwrap_or("No user token")}</p>
                <button {onclick}>{"Join the room"}</button>
                // <button {test_onclick}>{"Join the room"}</button>
            </div>
            </div>
        </div>
    }
}
