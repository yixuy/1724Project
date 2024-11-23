use actix_web::{web, App, HttpServer};
use user::routes::{auth, status};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/register", web::post().to(auth::register))
            .route("/login", web::post().to(auth::login))
            .route("/logout", web::post().to(auth::logout))
            .route("/status", web::get().to(status::get_status))
    })
        .bind(("localhost", 8080))?
        .run()
        .await
}
