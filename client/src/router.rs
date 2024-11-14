use crate::components::prelude::*;
use yew::{html, Html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/r/:room")]
    Room { room: String },
    #[at("/signup")]
    SignUp,
    #[at("/signin")]
    SignIn,
    #[at("/settings")]
    Settings,
    #[at("/logout")]
    LogOut,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <h1>{ "Home Page" }</h1> },
        Route::Room { room } => html! { <h1>{ format!("Room: {}", room) }</h1> },
        Route::SignUp => html! { <SignUp /> },
        Route::SignIn => html! { <SignIn /> },
        Route::Settings => html! { <Settings /> },
        Route::LogOut => html! { <LogOut /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
