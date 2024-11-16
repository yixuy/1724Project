use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(SignIn)]
pub fn sign_in() -> Html {
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

    // let go_to_first_post_button = {
    //     let navigator = navigator.clone();
    //     let onclick = Callback::from(move |_| navigator.push(&Route::Post { id: "first-post".to_string() }));
    //     html! {
    //         <button {onclick}>{"click to go the first post"}</button>
    //     }
    // };

    // let go_to_secure_button = {
    //     let onclick = Callback::from(move |_| navigator.push(&Route::Secure));
    //     html! {
    //         <button {onclick}>{"click to go to secure"}</button>
    //     }
    // };

    html! {
        <>
            {go_home_button}
        </>
    }
}
