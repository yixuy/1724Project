use crate::models::user::User;
use gloo::storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use yew::prelude::*;
const API_URL: &str = "http://127.0.0.1:5000";

pub async fn fetch_test_data(printed_information: UseStateHandle<String>) {
    let fetch_api_url = format!("{}/test", API_URL);
    match Request::get(&fetch_api_url).send().await {
        Ok(response) => match response.json::<String>().await {
            Ok(data) => {
                printed_information.set(data);
            }
            Err(err) => {
                gloo_console::error!("Failed to parse JSON:", err.to_string());
                printed_information.set("Error: Invalid JSON response.".to_string());
            }
        },
        Err(err) => {
            gloo_console::error!("Request failed:", err.to_string());
            printed_information.set("Error: Request failed.".to_string());
        }
    }
}

// fn get_current_user_state() -> User {
//     let username = get_current_user().unwrap();
//     let token = get_user_token(&username).unwrap();
//     let user = User::new(username, token);
//     user
// }
pub fn get_current_user() -> Option<String> {
    LocalStorage::get("current_user").ok()
}

pub fn get_user_token(username: &str) -> Option<String> {
    let user_key = format!("user_{}", username);
    LocalStorage::get(&user_key).ok()
}

pub async fn get_user(printed_information: UseStateHandle<String>) {
    let fetch_api_url = format!("{}/user", API_URL);
    let username = get_current_user().unwrap();
    let token = get_user_token(&username).unwrap().replace(r#"""#, "");
    let url_with_token = format!("{}/{}", fetch_api_url, token);
    gloo_console::log!("URL with token:", url_with_token.clone());
    match Request::get(&url_with_token).send().await {
        Ok(response) => match response.json::<User>().await {
            Ok(data) => {
                // gloo_console::log!("Data:", data.clone());
                let user_info = serde_json::to_string(&data).unwrap();
                printed_information.set(user_info);
            }
            Err(err) => {
                gloo_console::error!("Failed to parse JSON:", err.to_string());
                printed_information.set("Error: Invalid JSON response.".to_string());
            }
        },
        Err(err) => {
            gloo_console::error!("Request failed:", err.to_string());
            printed_information.set("Error: Request failed.".to_string());
        }
    }
}