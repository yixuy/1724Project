use actix_web::web::Data;
mod db;
mod user;
use db::Database;
mod error;
mod router;
use router::{create_user, get_user, get_users, test_handler, update_user};
mod user_trait;

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
    // Don’t forget to add the service to the app
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(test_handler)
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
