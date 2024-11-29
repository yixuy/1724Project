use crate::components::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/room/:room")]
    Room { room: String },
    #[at("/signup")]
    SignUp,
    #[at("/signin")]
    SignIn,
    #[at("/logout")]
    LogOut,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Room { room } => html! { <Room room={room} /> },
        Route::SignUp => html! { <SignUp /> },
        Route::SignIn => html! { <SignIn /> },
        Route::LogOut => html! { <LogOut /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
