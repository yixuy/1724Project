use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::services::status_service;

/* User status route handler */

#[derive(Deserialize)]
pub struct UserData {
    username: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    status: String,
}

pub async fn get_status(data: web::Json<UserData>) -> impl Responder {
    // fetch user status
    let status = status_service::fetch_status(&data.username).await?;
    Ok(HttpResponse::Ok().json(StatusResponse {
        status,
    }))
}
