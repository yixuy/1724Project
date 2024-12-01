mod components;
mod endpoints;
// mod error;
mod models;
mod router;
use endpoints::{get_current_user, get_user};

use models::user::User;
// use reqwasm::http::Request;
use router::{switch, Route};
// use serde_json;
use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let username = get_current_user().unwrap();
    let fetched = use_state(|| false);
    let user = User::new(username.clone(), "".to_string());
    let user_string = use_state(|| "".to_string());
    {
        let mut user = user.clone();
        let user_string_clone = user_string.clone();
        let fetched = fetched.clone();
        if *fetched == false {
            wasm_bindgen_futures::spawn_local(async move {
                get_user(user_string_clone.clone()).await;
                let user_json: User = serde_json::from_str(&*user_string_clone).unwrap();
                user.set_username(user_json.username);
                // user.set_username(user_json);
                fetched.set(true);
            });
        }
    }

    let css = style!(
        r#"

        .inner_container {
            background-color: green; /* Keeps your background color */
            height: 100vh; /* Ensures full height of the viewport */
            display: flex; /* Enables Flexbox */
            justify-content: center; /* Centers horizontally */
            align-items: center; /* Centers vertically */
            flex-direction: column; /* Stacks children vertically */
            text-align: center; /* Centers text inside the container */
        }

        .top-right-nav {
            position: absolute;
            top: 0;
            right: 0;
            margin: 50px;
            display: flex;
            gap: 20px; /* 20px margin between each link */
        }
        .top-left-nav {
            position: absolute;
            top: 0;
            left: 0;
            margin: 50px;
            display: flex;
            gap: 20px; /* 20px margin between each link */
        }

        .top-right-nav ul {
            list-style: none;
            display: flex;
            gap: 15px;
        }
        .card {
                margin: 50px auto; /* Center the card horizontally and add top/bottom margin */
                background-color: #4287f5; /* Card's background color */
                border-radius: 20px; /* Rounded corners */
                padding: 20px; /* Padding inside the card */
                width: 80%; /* Set width for the card */
                
                text-align: center; /* Center text inside the card */
                box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1); /* Optional: Add shadow for a card effect */
        }
        .top-right-nav a {
            text-decoration: none;
            color: black;
            font-weight: bold;
        }

        
        "#
    )
    .unwrap();

    #[function_component(NavBar)]
    pub fn nav_bar() -> Html {
        html! {
            <div >
                <BrowserRouter>
            <div class = "card" >
                <h1>{ "Welcome to the chat app" }</h1>
                        <nav class = "top-right-nav">

                            // <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                            <Link<Route> to={Route::SignIn}>{ "Sign In" }</Link<Route>>

                            <Link<Route> to={Route::SignUp}>{ "Sign Up" }</Link<Route>>
                            // <Link<Route> to={Route::Settings}>{ "Settings" }</Link<Route>>
                            // <Link<Route> to={Route::LogOut}>{ "Log Out" }</Link<Route>>
                        </nav>
            </div>
                        <hr />
                        <Switch<Route> render={switch} />

                    </BrowserRouter>
                    <divider/>
        </div>
            }
    }

    let printed_information = use_state(|| "nothing".to_string());

    // let printed_information = printed_information
    let onclick = {
        let printed_information = printed_information.clone();
        Callback::from(move |_| {
            let printed_information = printed_information.clone();
            wasm_bindgen_futures::spawn_local(async move {
                endpoints::fetch_test_data(printed_information).await;
            });
        })
    };

    html! {
        <div class={css.get_class_name().to_string()}>
            <div class="container">
                
                <NavBar />
                <divider/>
                <div class="top-left-nav">
                    <button {onclick}>{"Test"}</button>
                    <br/>
                    <p  >{ (*printed_information).clone() }</p>
                    if username != "" {
                        <h2>{ format!("Welcome, {}!", (*user_string).clone()) }</h2>
                    } else {
                        <h2>{ "Please Sign up the username before you can join the room" }</h2>
                    }
                </div>


            </div>
        </div>
    // </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}