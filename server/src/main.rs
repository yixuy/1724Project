use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{http, web, App, HttpServer};
mod db;
mod models;
use db::Database;
use actix::Actor;
mod auth;
mod endpoints;
mod error;
mod server;
mod services;
use endpoints::*;
// mod user_trait;

const BACKEND_URL: &str = "127.0.0.1:5000";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to the database
    let db = Database::init()
        .await
        .expect("Failed to connect to the database");
    let server = server::ChatServer::new().start();
    let db_data = Data::new(db);

    // Check point
    // Don’t forget to add the service to the app
    
    HttpServer::new(move || {
        // Allow cors for the server
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(server.clone()))
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
