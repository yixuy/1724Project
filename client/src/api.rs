use reqwasm::http::Request;
use yew::prelude::*;
const API_URL: &str = "http://127.0.0.1:5000/test";

pub async fn fetch_data(printed_information: UseStateHandle<String>) {
    match Request::get(API_URL).send().await {
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
