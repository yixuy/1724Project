use crate::components::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/user/:username")]
    User { username: String },
    #[at("/room/:username/:room")]
    Room { username: String, room: String },
    #[at("/signup")]
    SignUp,
    #[at("/signin")]
    SignIn,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::User { username } => html! { <UserComponent username={username} /> },
        Route::Room { username, room } => html! { <Room username = {username} room_id={room} /> },
        Route::SignUp => html! { <SignUp /> },
        Route::SignIn => html! { <SignIn /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
