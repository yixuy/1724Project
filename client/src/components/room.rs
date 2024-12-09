use crate::endpoints::get_user;
use crate::models::message::Message;
use crate::models::room::RoomAttribute;
use crate::models::user::User;
use futures_util::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message as WsMessage};
use serde_json;
use std::cell::RefCell;
use std::rc::Rc;
use stylist::style;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(Room)]
pub fn room(RoomAttribute { username, room_id }: &RoomAttribute) -> Html {
    let messages = use_state(|| vec![]);
    let messages_for_receive = messages.clone();
    let messages_for_send = messages.clone();
    let message_input = use_state(|| String::new());
    let writer = use_state(|| Rc::new(RefCell::new(None)));
    let writer_for_effect = writer.clone();
    let username_clone = username.clone();
    let username_display = username.clone();
    let room_id_clone = room_id.clone();
    let room_id = room_id.clone();
    let has_run = use_state(|| false);

    use_effect(move || {
        let messages_for_effect = messages_for_receive.clone();
        let username = username_clone.clone();
        let room_id = room_id.clone();
        let has_run = has_run.clone();
        gloo_console::log!(format!("move{:?}", messages_for_effect.clone()));
        if !*has_run {
            let ws_url = format!("ws://127.0.0.1:5000/ws/{}/{}", username, room_id);
            let ws = WebSocket::open(&ws_url).expect("Failed to connect to WebSocket");

            let (w, mut r) = ws.split();
            writer_for_effect.set(Rc::new(RefCell::new(Some(w))));
            let messages_for_effect = messages_for_effect.clone();
            gloo_console::log!(format!("spawn_local{:?}", messages_for_effect.clone()));
            spawn_local(async move {
                let messages_for_effect = messages_for_effect.clone();
                while let Some(Ok(WsMessage::Text(text))) = r.next().await {
                    // gloo_console::log!(format!("while{:?}", messages_for_effect.clone()));
                    // gloo_console::log!("Received a message:", &text);

                    if let Some((prefix, json_part)) = text.split_once(':') {
                        let prefix = prefix.trim();
                        let json_str = json_part.trim();
                        if prefix.to_string() != username {
                            let msg = Message {
                                username: prefix.to_string(),
                                content: serde_json::from_str::<Message>(json_str).unwrap().content,
                            };
                            let json_msg = serde_json::to_string(&msg).unwrap();
                            gloo_console::log!(format!("COn{:?}", json_str));

                            if let Ok(received_msg) =
                                serde_json::from_str::<Message>(json_msg.as_str())
                            {
                                let mut new_messages = (*messages_for_effect).clone();
                                new_messages.push(received_msg);
                                messages_for_effect.set(new_messages);
                            } else {
                                gloo_console::log!("Failed to parse message");
                            }
                        }
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
        let messages = messages_for_send.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let msg_input = (*message_input).clone();
            let writer_for_async = writer.clone();
            let uname = username.clone();
            let message_input_for_async = message_input.clone();
            let messages_for_async = messages.clone();

            spawn_local(async move {
                if !msg_input.is_empty() {
                    if let Some(w) = writer_for_async.borrow_mut().as_mut() {
                        // Construct the message to send
                        let msg = Message {
                            username: uname.clone(),
                            content: msg_input.clone(),
                        };

                        // Serialize message to JSON
                        if let Ok(msg_json) = serde_json::to_string(&msg) {
                            // Send over WebSocket
                            if let Err(e) = w.send(WsMessage::Text(msg_json)).await {
                                gloo_console::log!("Failed to send message:", &format!("{:?}", e));
                            } else {
                                // Since message sent successfully, update UI immediately
                                let mut new_messages = (*messages_for_async).clone();
                                new_messages.push(msg);
                                // gloo_console::log!(format!("{:?}", new_messages));
                                messages_for_async.set(new_messages);
                            }
                        }

                        // Clear the input box
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
            flex-grow: 0;
            overflow-y: auto;
            margin: 10px;
            padding: 10px;
            border: 1px solid #ccc;
            border-radius: 5px;
            background-color: lightblue;
            height: 50vh;
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
    html! {
        <>
            <div class={css.get_class_name().to_string()}>
                    <div class = "title">
                    <h1>{ format!("User {} in Room {}", username_display, room_id_clone ) }</h1>

                    </div>

            <div class="messages">
                { for ((*messages).clone()).iter().map(|message| html! {
                    <div class="message">
                        <span>{ format!("{}[{}]:", &message.username, "status")}</span>
                        <br />
                        <strong>{ &message.content }</strong>
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
