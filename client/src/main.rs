// // use yew::prelude::*;

// // // #[derive(Serialize, Deserialize, Debug)]
// // // pub struct Message {
// // //     pub text: String,
// // //     pub from: String,
// // // }

// // // Logic component

// // // pub struct Model {
// // //     alias: String,

// // //     messages: Vec<Message>,

// // // }

// // #[function_component]
// // fn App() -> Html {
// //     let counter = use_state(|| 0);
// //     let onclick = {
// //         let counter = counter.clone();
// //         move |_| {
// //             let value = *counter + 1;
// //             counter.set(value);
// //         }
// //     };

// //     html! {
// //         <div>
// //             <button {onclick}>{ "+1" }</button>
// //             <p>{ *counter }</p>
// //         </div>
// //     }
// // }

// // use components::prelude::*;

// mod router;
// use router::Route;
// use router::switch;


// use yew::prelude::*;
// use yew_router::prelude::*;

// // #[derive(Clone, Routable, PartialEq)]
// // enum Route {
// //     #[at("/")]
// //     Home,
// //     #[at("/signin")]
// //     SignIn,
// //     #[at("/signup")]
// //     SignUp,
// //     #[not_found]
// //     #[at("/404")]
// //     NotFound,
// // }

// // #[function_component(SignUp)]
// // fn signup() -> Html {
// //     let navigator = use_navigator().unwrap();

// //     let onclick = Callback::from(move |_| navigator.push(&Route::Home));
// //     html! {
// //         <div>
// //             <h1>{ "SignUp" }</h1>
// //             <button {onclick}>{ "Go Home" }</button>
// //         </div>
// //     }
// // }

// // fn switch(routes: Route) -> Html {
// //     match routes {
// //         Route::Home => html! { <h1>{ "Home" }</h1> },
// //         Route::SignIn => html! {
// //             <signin />
// //         },
// //         Route::SignUp => html! {
// //             <SignUp />
// //         },
// //         Route::NotFound => html! { <h1>{ "404" }</h1> },
// //     }
// // }

// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         <BrowserRouter>
//             <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
//         </BrowserRouter>
//     }
// }

// fn main() {
//     yew::Renderer::<App>::new().render();
// }


mod components;
mod router;
use router::{Route, switch};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav>
                <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                <Link<Route> to={Route::SignIn}>{ "Sign In" }</Link<Route>>
                <Link<Route> to={Route::SignUp}>{ "Sign Up" }</Link<Route>>
                <Link<Route> to={Route::Settings}>{ "Settings" }</Link<Route>>
                <Link<Route> to={Route::LogOut}>{ "Log Out" }</Link<Route>>
            </nav>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
