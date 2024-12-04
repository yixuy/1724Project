use crate::{models::user::User, router::Route};
use stylist::style;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(SignUp)]
pub fn sign_in() -> Html {
    let navigator = use_navigator().unwrap();
    let css = style!(
        r#"
    .SignUp-container {
    background-color: #4287f5; /* Card's background color */
    display: flex; /* Enable flexbox */
    justify-content: center; /* Center horizontally */
    align-items: center; /* Center vertically */
    height: 100vh; /* Full height of the viewport */
    width: 100vw; /* Full width of the viewport */
    box-sizing: border-box; /* Include padding and border in dimensions */
    margin: 0; /* Remove any extra margins */
}

.SignUp-form {
    background: white; /* Background color of the form */
    padding: 20px; /* Padding inside the form */
    border-radius: 8px; /* Rounded corners */
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); /* Add shadow */
    text-align: center; /* Center the text */
    width: 90%; /* Responsive width */
    max-width: 300px; /* Restrict the form's maximum size */
}

    input[type="text"],
    input[type="password"] {
        width: 80%;
        padding: 10px;
        margin-bottom: 10px;
        border: 1px solid #ccc;
        border-radius: 4px;
    }

    button {
        width: 70%;
        padding: 10px;
        color: white;
        background-color: #007BFF;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    button:hover {
        background-color: #0056b3;
    }
    "#
    )
    .unwrap();
    let go_home_button = {
        let navigator = navigator.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Home));

        html! {
            <div >
            <h1 >{ "Sign Up Page" }</h1>
            <button {onclick}>{"click to go home"}</button>
        </div>
        }
    };

    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    // let message = use_state(|| "".to_string());

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        // let message = message.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let data = User::new((*username).clone(), (*password).clone());
            let navigator = navigator.clone();
            // fetch_base_url(data, navigator);
            wasm_bindgen_futures::spawn_local(async move {
                match reqwest::Client::new()
                    .post("http://127.0.0.1:5000/new_user")
                    .json(&data)
                    .send()
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        // message.set("Sign-in successful!".to_string());
                        navigator.push(&Route::SignIn);
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
      <div class={css.get_class_name().to_string()}>
        <div class="SignUp-container">
            <form onsubmit={Some(on_submit)} class="SignUp-form">
            <div >
                <h1>{"Sign Up"}</h1>
                <input
                    type="text"
                    placeholder="Username"
                    value={(*username).clone()}
                    oninput={on_username_change}
                />
                <br/>
                <input
                    type="password"
                    placeholder="Password"
                    value={(*password).clone()}
                    oninput={on_password_change}
                />
                <br/>
                <button type="submit">{"Sign Up"}</button>
                </div>
            </form>
            // <p>{(*message).clone()}</p>
        </div>
    </div>
    }
}
