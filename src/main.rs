mod server;

use actix_web::{App, HttpServer, web};
use actix::Actor; 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let server = server::ChatServer::new().start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws/", web::get().to(server::ws_index))
    })
    .bind("127.0.0.1:8080")? // 启动 WebSocket 8080
    .run()
    .await
}


