use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div>
            <h1>{ "404 Page Not Found" }</h1>
        </div>
    }
}
