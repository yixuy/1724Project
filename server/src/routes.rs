// REST API 
use std::time::Instant;
use actix::*;
use actix_files::NamedFile;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use serde_json::json;
use uuid::Uuid;
use crate::db;
use crate::models;
use crate::server;
use crate::session;
// type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;



// #[post("/users/create")]










// #[get("/users")]