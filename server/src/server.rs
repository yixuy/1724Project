use actix_web::{get, post, web, Error, HttpResponse, Responder};

#[get("/user")]
async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(get_user))
        .bind("127.0.0.1:8080")?
        .run().await()
}
