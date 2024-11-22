use crate::router::Route;
use stylist::style;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let css = style!(
        r#"
        .container {
            background-color: green; /* Keeps your background color */
            height: 100vh; /* Ensures full height of the viewport */
            display: flex; /* Enables Flexbox */
            justify-content: center; /* Centers horizontally */
            align-items: center; /* Centers vertically */
            flex-direction: column; /* Stacks children vertically */
            text-align: center; /* Centers text inside the container */
        }
       .card {
        background-color: #4287f5;  /* Card's background color */
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

    // let test_onclick = Callback::from(move |_| {
    //     wasm_bindgen_futures::spawn_local(async move {
    //         let response = Request::get("http://127.0.0.1:5000/test")
    //             .send()
    //             .await
    //             .unwrap();
    //         gloo_console::log!("Response received:", response.text().await.unwrap());
    //     });
    // });

    html! {
         <div class={css.get_class_name().to_string()}>
        //  <div class="container">
            <div class="card">
                // <h1>{ "Please login the application" }</h1>
                <h2>{ "Please Sign up the username before you can join the room" }</h2>
                <input value={room_number.to_string()} {oninput} />
                <p>{ format!("Room number: {}", *room_number) }</p>
                <button {onclick}>{"Join the room"}</button>
                // <button {test_onclick}>{"Join the room"}</button>
            </div>
            // </div>
        </div>
    }
}
