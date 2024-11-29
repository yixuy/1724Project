use crate::router::Route;
use serde::Serialize;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Serialize)]
struct SignUpData {
    username: String,
    password: String,
}
#[function_component(SignUp)]
pub fn sign_up() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));

        html! {
            <div >
            <h1>{ "Sign In Page" }</h1>
            <button {onclick}>{"click to go home"}</button>
        </div>
        }
    };

    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        // let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let data = SignUpData {
                username: (*username).clone(),
                password: (*password).clone(),
            };
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match reqwest::Client::new()
                    .post("http://127.0.0.1:5000/new_user")
                    .json(&data)
                    .send()
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        // message.set("Sign-in successful!".to_string());
                        navigator.push(&Route::Home);
                    }
                    _ => {
                        // message.set("Sign-in failed!".to_string());
                    }
                }
            });
        })
    };

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    html! {
        <div class="signin-container">
            <form onsubmit={Some(on_submit)} class="signin-form">
                <h1>{"Sign Up"}</h1>
                <input
                    type="text"
                    placeholder="Username"
                    value={(*username).clone()}
                    oninput={on_username_change}
                />
                <input
                    type="password"
                    placeholder="Password"
                    value={(*password).clone()}
                    oninput={on_password_change}
                />
                <button type="submit">{"Sign In"}</button>
            </form>
            // <p>{(*message).clone()}</p>
        </div>
    }
}
