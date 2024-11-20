// REST API 
use std::time::Instant;
use actix::*;
use actix_files::NamedFile;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use serde_json::json;
use uuid::Uuid;
use crate::db;
use crate::models;
use crate::server;
use crate::session;
// type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


#[post("/users/create")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_user(&mut conn, &form.username, &form.phone)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users/login")]
pub async fn login_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::LoginUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::login_user(&mut conn, &form.username, &form.password)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users/logout")]
pub async fn logout_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::LogoutUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::logout_user(&mut conn, &form.username)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users/logout")]
pub async fn logout_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::LogoutUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::logout_user(&mut conn, &form.username)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(user))
}
#[get("/users")]
pub async fn logout_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::LogoutUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::logout_user(&mut conn, &form.username)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;
    Ok(HttpResponse::Ok().json(user))
}