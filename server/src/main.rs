use actix_cors::Cors;
use actix_web::web::Data;
mod db;
mod user;
use db::Database;
mod error;
mod router;
use router::{create_user, get_user, get_users, test_handler, update_user};
mod user_trait;
use actix_web::{App, HttpServer};
const BANK_END_URL: &str = "127.0.0.1:5000";
#[actix_web::main]

async fn main() -> std::io::Result<()> {
    // Connect to the database
    let db = Database::init()
        .await
        .expect("Failed to connect to the database");
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
            .app_data(db_data.clone())
            .service(test_handler)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(get_users)
    })
    .bind(BANK_END_URL)?
    .run()
    .await
    .map_err(|e| {
        eprintln!("Server error: {}", e);
        e
    })
}
