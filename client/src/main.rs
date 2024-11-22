mod api;
mod components;
mod router;

// use reqwasm::http::Request;
use router::{switch, Route};
// use serde_json;
use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // request::get_request();

    let css = style!(
        r#"
        .container {
            background-color: lightblue;
            height: 100vh;
            display: flex;
            justify-content: center;
            
        }

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

    // let onclick = Callback::from(move |_| {
    //     wasm_bindgen_futures::spawn_local(async move {
    //         let response = Request::get("http://127.0.0.1:5000/test")
    //             .send()
    //             .await
    //             .unwrap();
    //         // print!("The response is: {:?}", response.status());
    //     });
    // });
    let printed_information = use_state(|| "nothing".to_string());

    // let printed_information = printed_information
    let onclick = {
        let printed_information = printed_information.clone();
        Callback::from(move |_| {
            let printed_information = printed_information.clone();
            wasm_bindgen_futures::spawn_local(async move {
                api::fetch_data(printed_information).await;
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
                </div>
                    // <div class="inner_container">
                    //     <h1>{ "Welcome sto the chat app" }</h1>
                    //     <p>{ "Please Register the username before you can join the room" }</p>
                    //     <input />
                    //     <button>{"Join the room"}</button>
                    // <div/>
                // <ChatArea />
                // <TypeArea />

            </div>
        </div>
    // </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
