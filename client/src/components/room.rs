use crate::models::message::Message;
use crate::models::room::RoomId;
use stylist::style;
use yew::prelude::*;

#[function_component(Room)]
pub fn room(RoomId { room_id }: &RoomId) -> Html {
    // let user = get_user();

    let messages = use_state(|| vec![]);
    let username = use_state(|| String::from("User"));
    let message_input = use_state(|| String::new());
    // let on_submit = {
    //     let username = username.clone();
    //     let password = password.clone();
    //     // let message = message.clone();

    //     Callback::from(move |e: SubmitEvent| {
    //         e.prevent_default();
    //         let data = User::new((*username).clone(), (*password).clone());
    //         let navigator = navigator.clone();

    //         wasm_bindgen_futures::spawn_local(async move {
    //             match reqwest::Client::new()
    //                 .post("http://127.0.0.1:5000/login")
    //                 .json(&data)
    //                 .send()
    //                 .await
    //             {
    //                 Ok(response) if response.status().is_success() => {
    //                     if let Ok(body) = response.text().await {
    //                         // Store the response in a cookie
    //                         LocalStorage::set("current_user", data.username.clone())
    //                             .expect("Failed to set current user");
    //                         let user_key = format!("user_{}", data.username.clone());
    //                         LocalStorage::set(user_key, body.clone())
    //                             .expect("Failed to set cookie");
    //                         web_sys::console::log_1(&format!("Stored in cookie: {}", body).into());

    //                         // Navigate to the Home page
    //                         navigator.push(&Route::Home);
    //                         if let Some(window) = window() {
    //                             window.location().reload().unwrap(); // Refresh the page
    //                         }
    //                     }
    //                 }
    //                 _ => {
    //                     web_sys::console::log_1(&"Sign-in failed!".into());
    //                     // message.set("Sign-in failed!".to_string());
    //                 }
    //             }
    //         });
    //     })
    // };

    let css = style! {
        r#"
        body {
            margin: 0;
            font-family: Arial, sans-serif;
            display: flex;
            flex-direction: column;
            height: 100vh;
        }

        .chatbox {
            display: flex;
            
            flex-direction: column;
            justify-content: space-between;
            height: 100%;
        }

        .messages {
            flex-grow: 0; /* Disable automatic growing to allow fixed height */
            overflow-y: auto;
            margin: 10px;
            padding: 10px;
            border: 1px solid #ccc;
            border-radius: 5px;
            background-color: lightblue;
            height: 50vh; /* Set the height to 50% of the viewport height */
        }

        .message {
            margin-bottom: 10px;
        }
        .title {
            display: flex;
            justify-content: center;
            height: 100%;
        }

        form {
            display: flex;
            padding: 10px;
            border-top: 1px solid #ccc;
            background-color: #fff;
        }

        input[type="text"] {
            flex-grow: 1;
            margin-right: 10px;
            padding: 10px;
            font-size: 16px;
            border: 1px solid #ccc;
            border-radius: 5px;
            background-color: #f9f9f9;
        }

        button {
            padding: 10px 20px;
            font-size: 16px;
            background-color: #007BFF;
            color: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
        }

        button:hover {
            background-color: #0056b3;
        }
        "#
    }
    .unwrap();
    let on_message_submit = {
        let messages = messages.clone();
        let message_input = message_input.clone();
        let username = username.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default(); // Prevent page reload on form submission
            if !message_input.is_empty() {
                let mut new_messages = (*messages).clone();
                new_messages.push(Message {
                    username: (*username).clone(),
                    content: (*message_input).clone(),
                    timestamp: String::from("now"),
                });
                messages.set(new_messages);
                message_input.set(String::new());
            }
        })
    };

    let on_input_change = {
        let message_input = message_input.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                message_input.set(input.value());
            }
        })
    };

    html! {

        <>
        <div class={css.get_class_name().to_string()}>
            <div class = "title">
                <h1>{ format!("Room Page {}", room_id) }</h1>
            </div>
            <div class="messages">
                { for (*messages).iter().map(|message| html! {
                    <div class="message">
                        <span>{ format!("{}: [status]", &message.username)}</span>
                        <br />
                        <strong>{  &message.content }</strong>


                    </div>
                })}
            </div>
            <form onsubmit={on_message_submit}>
                <input
                    type="text"
                    value={(*message_input).clone()}
                    oninput={on_input_change}
                    placeholder="Type your message here..."
                />
                <button type="submit">{ "Send" }</button>
            </form>
        </div>
        </>
    }
}
