use crate::endpoints::{get_current_user, get_room, get_user};
use crate::models::user::User;
use crate::router::Route;
use stylist::style;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
#[function_component(Home)]
pub fn home() -> Html {
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

    let username = get_current_user().unwrap_or_else(|| "".to_string());
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
    let room_number = use_state(|| 0);
    let is_room_created = use_state(|| false);
    let oninput = {
        let room_number = room_number.clone();
        let is_room_created = is_room_created.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
            {
                let is_room_created = is_room_created.clone();

                if let Ok(num) = input.value().parse::<i32>() {
                    room_number.set(num);
                    let room_number = num.to_string();
                    wasm_bindgen_futures::spawn_local(async move {
                        if get_room(&room_number).await.is_some() {
                            is_room_created.set(true);
                        } else {
                            is_room_created.set(false);
                        }
                    });
                }
            }
        })
    };

    html! {
         <div class={css.get_class_name().to_string()}>
         <div class="container">
            <nav class = "top-right-nav">
                            <Link<Route> to={Route::SignIn}>{ "Sign In" }</Link<Route>>

                            <Link<Route> to={Route::SignUp}>{ "Sign Up" }</Link<Route>>
            </nav>
            <div class="card">

                    <h2>{ "Please Sign up the username before you can join the room" }</h2>


                <h2>{ format!("What is the room number you want to join?")}</h2>
                <input value={room_number.to_string()} {oninput} />
                <br />
                if *is_room_created {
                    <p>{ format!("Chat Room {} found", room_number.to_string()) }</p>
                    // <button {onclick}>{"Join the room"}</button>
                }else{
                    <p>{ format!("Chat Room {} not found", room_number.to_string()) }</p>

                }
            </div>
            </div>
        </div>
    }
}
