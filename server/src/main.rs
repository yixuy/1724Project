// use serde::{Deserialize, Serialize};
// use surrealdb::engine::remote::ws::Ws;
// use surrealdb::opt::auth::Root;
// use surrealdb::Surreal;

// #[derive(Debug, Serialize, Deserialize)]
// struct User {
//     first_name: String,
//     last_name: String,

// }

use std::fmt::format;

use actix_web::web::Data;
// use actix_web::dev::Path;
use actix_web::{get, patch, post, web, web::Json, web::Path, HttpResponse, Responder};
mod user;
use user::NewUser;
use user::UpdateUserURL;
use uuid::Uuid;
use validator::Validate;
mod db;
use db::Database;
mod error;
use error::UserError;
mod router;
use router::{create_user, get_user, get_users, test_handler, update_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to the server
    use actix_web::{App, HttpServer};
    // Connect to the database
    let db = Database::init()
        .await
        .expect("Failed to connect to the database");
    let db_data = Data::new(db);

    // Check point
    // Don;t forget to add the service to the app
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(router::test_handler)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(get_users)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
    .map_err(|e| {
        eprintln!("Server error: {}", e);
        e
    })
}
