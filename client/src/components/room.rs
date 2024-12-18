use crate::endpoints::*;
use crate::models::message::*;
use crate::models::prelude::*;
use crate::router::Route;
use futures_util::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message as WsMessage};
use serde_json;
use std::cell::RefCell;
use std::rc::Rc;
use stylist::style;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Room)]
pub fn room(RoomAttribute { username, room_id }: &RoomAttribute) -> Html {
    let messages = use_state(|| vec![]);
    let status: UseStateHandle<Vec<String>> = use_state::<Vec<String>, _>(|| vec![]);
    let status_clone = status.clone();
    let messages_for_receive = messages.clone();
    let message_input = use_state(|| String::new());
    let writer = use_state(|| Rc::new(RefCell::new(None)));
    let writer_for_effect = writer.clone();
    let username_clone = username.clone();
    let username_display = username.clone();
    let room_id_clone = room_id.clone();
    let room_id = room_id.clone();
    let has_run = use_state(|| false);

    use_effect(move || {
        let messages_for_effect = messages_for_receive;
        let username = username_clone.clone();
        let room_id = room_id.clone();
        let has_run = has_run.clone();

        if !*has_run {
            let ws_url = format!("ws://127.0.0.1:5000/ws/{}/{}", username, room_id);
            let ws = WebSocket::open(&ws_url).expect("Failed to connect to WebSocket");

            let (w, mut r) = ws.split();
            writer_for_effect.set(Rc::new(RefCell::new(Some(w))));

            spawn_local(async move {
                let messages_for_effect = messages_for_effect.clone();
                while let Some(Ok(WsMessage::Text(text))) = r.next().await {
                    if text == "leave" {
                        if let Some(window) = web_sys::window() {
                            window.location().reload().unwrap_or_else(|err| {
                                gloo_console::log!("Failed to reload page:", &format!("{:?}", err));
                            });
                        }
                    } else {
                        let raw_messages: Vec<ChatMessage> = match serde_json::from_str(&text) {
                            Ok(messages) => messages,
                            Err(e) => {
                                gloo_console::log!("Failed to parse message:", &format!("{:?}", e));
                                continue;
                            }
                        };

                        let mut temp_status_futures = vec![];
                        if raw_messages.is_empty() {
                            continue;
                        }
                        for msg in raw_messages.iter() {
                            let username = msg.username.clone();
                            temp_status_futures.push(async move {
                                get_user_status_by_username(username).await.unwrap()
                            });
                        }
                        let temp_status: Vec<String> =
                            futures::future::join_all(temp_status_futures).await;
                        status.set(temp_status);
                        messages_for_effect.set(raw_messages);
                    }
                }
            });
            has_run.set(true);
        }
        || {}
    });

    let on_message_submit = {
        let message_input = message_input.clone();
        let writer = writer.clone();
        let username = username.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let msg_input = (*message_input).clone();
            let writer_for_async = writer.clone();
            let uname = username.clone();
            let message_input_for_async = message_input.clone();

            spawn_local(async move {
                if !msg_input.is_empty() {
                    if let Some(w) = writer_for_async.borrow_mut().as_mut() {
                        // Construct the message to send
                        let msg = NewChatMessage {
                            username: uname.clone(),
                            content: msg_input.clone(),
                        };

                        if let Ok(msg_json) = serde_json::to_string(&msg) {
                            // Send over WebSocket
                            if let Err(e) = w.send(WsMessage::Text(msg_json)).await {
                                gloo_console::log!("Failed to send message:", &format!("{:?}", e));
                            }
                        }

                        message_input_for_async.set(String::new());
                    } else {
                        gloo_console::log!("WebSocket not connected");
                    }
                }
            });
        })
    };

    // Handle input changes
    let on_input_change = {
        let message_input = message_input.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                message_input.set(input.value());
            }
        })
    };

    let navigator = use_navigator().unwrap();
    let on_leave = {
        let writer = writer.clone(); // WebSocket writer
        let username = username.clone();
        let room_id = room_id_clone.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let writer = writer.clone();
            let username = username.clone();
            let room_id = room_id.clone();
            let navigator = navigator.clone();

            spawn_local(async move {
                if let Some(w) = writer.borrow_mut().as_mut() {
                    // Construct the leave message
                    let leave_msg = serde_json::json!({
                        "action": "leave",
                        "username": username.clone(),
                        "room_id": room_id.clone()
                    });

                    // Serialize the leave message to JSON
                    if let Ok(msg_json) = serde_json::to_string(&leave_msg) {
                        // Send the message over WebSocket
                        if let Err(e) = w.send(WsMessage::Text(msg_json)).await {
                            gloo_console::log!(
                                "Failed to send leave message:",
                                &format!("{:?}", e)
                            );
                        }
                    }
                }
                // Close WebSocket connection
                writer.borrow_mut().take();

                // Navigate back to the user route (or another page)
                navigator.push(&Route::User { username });
            });
        })
    };

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
            flex: 1; /* Allows the box to expand and take available space */
            overflow-y: auto; /* Scroll when content exceeds the container */
            margin: 10px;
            padding: 10px;
            border: 1px solid #ccc;
            border-radius: 5px;
            background-color: lightblue;
            min-height: 50vh; /* Minimum height for better visibility */
        }

        .message {
            border: 2px solid white;
            border-radius: 5px;
            padding: 10px;
            margin: 5px;
            background-color: #f0f8ff;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
            word-break: break-word; /* Ensure long words wrap correctly */
        }

        .title {
            display: flex;
            align-items: center; /* Vertically centers content */
            justify-content: center; /* Horizontally centers content */
            gap: 10px; /* Adds spacing between the title and the logout button */
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
        .logout-button {
            height: 50%;
            margin-left: auto; /* Push the button to the far right in a flex container */
            padding: 5px 10px;
            font-size: 14px;
            background-color: #FF4C4C;
            color: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
        }

        .logout-button:hover {
            background-color: #CC0000;
}
        "#
    }
    .unwrap();
    html! {
        <>
            <div class={css.get_class_name().to_string()}>
                <div class="title">
                    <h1>{ format!("User {} in Room {}", username_display, room_id_clone) }</h1>
                </div>
                <div class="chatbox">
                    <button onclick={on_leave} class="logout-button">{ "Leave Room" }</button>
                </div>

                <div class="messages">
                    { for messages.iter().zip(status_clone.iter()).map(|(message, status)| html! {
                        <div class="message">
                            <span>
                                { format!("Username: {} ----- Status: [", &message.username) }
                                    <span
                                        style={if status == "Online" {
                                            "color: green; font-weight: bold;"
                                        } else if status == "Offline" {
                                            "color: red;"
                                        } else {
                                            "color: gray;"
                                        }}
                                    >
                                        { status }
                                    </span>
                                    { format!("] ----- {}", message.timestamp) }
                            </span>
                            <p>{ &message.content }</p>
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
