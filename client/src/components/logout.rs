use serde::Serialize;
use yew::prelude::*;

#[derive(Serialize)]
struct LogOutData {
    token: String,
}

#[function_component(LogOut)]
pub fn log_out() -> Html {
    html! {
        <div>
            <h1>{ "Log Out Page" }</h1>
        </div>
    }
}
