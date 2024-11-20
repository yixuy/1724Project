// use serde::{Deserialize, Serialize};
// use surrealdb::engine::remote::ws::Ws;
// use surrealdb::opt::auth::Root;
// use surrealdb::Surreal;

// #[derive(Debug, Serialize, Deserialize)]
// struct User {
//     first_name: String,
//     last_name: String,

// }

use actix_web::{
    body, get, post,
    web::{self, Json},
    Error, HttpResponse, Responder,
};
mod user;
use user::NewUser;
use validator::Validate;

#[get("/user/{id}")]
async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("User")
}

#[post("/user")]
async fn create_user(body: Json<NewUser>) -> impl Responder {
    // Create a new user with id and password
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let user = body.username.clone();
            HttpResponse::Ok()
                .body("User create successfully, the username is: ".to_string() + &user)
        }
        Err(e) => HttpResponse::BadRequest().body("User not created"),
    }
}

#[post("/user/{uuid}")]
async fn update_user() -> impl Responder {
    HttpResponse::Ok().body("Update User")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to the server
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(get_user)
            .service(create_user)
            .service(update_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(|e| {
        eprintln!("Server error: {}", e);
        e
    })
    // let db = Surreal::new::<Ws>("127.0.0.1:8001").await?;

    // // Signin as a namespace, database, or root user
    // db.signin(Root {
    //     username: "root",
    //     password: "root",
    // })
    // .await?;

    // // Name database
    // db.use_ns("test").use_db("test").await?;

    // // Create a new person with a random id
    // let created: Option<User> = db
    //     .create("user")
    //     .content(User {
    //         first_name: "Tobie".to_string(),
    //         last_name: "Morgan Hitchcock".to_string(),
    //     })
    //     .await?;
    // if let Some(record) = created {
    //     dbg!(record);
    // } else {
    //     eprintln!("No record created");
    // }

    // Ok(())
}

// #[actix_web::main]
// fn main() -> {

// }
