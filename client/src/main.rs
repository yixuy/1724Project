mod components;
mod router;
use router::{switch, Route};
use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;
// use crate::router::Route;

#[function_component(App)]
fn app() -> Html {
    let css = style!(
        r#"
        .container {
            background-color: lightblue;
            height: 100vh;
            padding: 20px;
        }

        .top-right-nav {
            position: absolute;
            top: 0;
            right: 0;
            margin: 20px;
            display: flex;
            gap: 20px; /* 20px margin between each link */
        }

        .top-right-nav ul {
            list-style: none;
            display: flex;
            gap: 15px;
        }

        .top-right-nav a {
            text-decoration: none;
            color: black;
            font-weight: bold;
        }
        "#
    )
    .unwrap();



    html! {
    <div style="background-color: lightblue; height: 100vh;">
        <BrowserRouter>
            <div  class={css.get_class_name().to_string()}>
            <nav class = "top-right-nav">
                // <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                <Link<Route> to={Route::SignIn}>{ "Sign In" }</Link<Route>>

                <Link<Route> to={Route::SignUp}>{ "Sign Up" }</Link<Route>>
                // <Link<Route> to={Route::Settings}>{ "Settings" }</Link<Route>>
                // <Link<Route> to={Route::LogOut}>{ "Log Out" }</Link<Route>>
            </nav>
            </div>

            <Switch<Route> render={switch} />
            
        </BrowserRouter>
    </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
