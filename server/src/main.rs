use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
mod db;
mod models;
use actix::Actor;
use db::Database;
mod auth;
mod endpoints;
mod error;
mod server;
mod services;
use endpoints::*;

const BACKEND_URL: &str = "127.0.0.1:5000";
const FRONT_URL: &str = "http://127.0.0.1:8080";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to the database
    let db: Database = Database::init()
        .await
        .expect("Failed to connect to the database");
    let db_data = Data::new(db.clone());
    let server = Data::new(server::ChatServer::new(db_data.clone()).start());

    // Check point
    // Don’t forget to add the service to the app

    HttpServer::new(move || {
        // Allow cors for the server
        let cors = Cors::default()
            .allowed_origin(FRONT_URL)
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(server.clone())
            .app_data(db_data.clone())
            .service(test_handler)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(get_users)
            .service(login_user)
            .service(create_room)
            .service(get_rooms)
            .service(get_room)
            .service(get_user_status)
            .route("/ws/{username}/{room_id}", web::get().to(server::ws_index))
    })
    .bind(BACKEND_URL)?
    .run()
    .await
    .map_err(|e| {
        eprintln!("Server error: {}", e);
        e
    })
}
