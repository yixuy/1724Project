use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Debug, Clone)]
pub struct Database {
    pub client: Surreal<Client>,
}

// Database for the chat app
impl Database {
    pub async fn init() -> Result<Self, Error> {
        // connect to the database
        let client = Surreal::new::<Ws>("127.0.0.1:5050").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        client.use_ns("surreal").use_db("chat").await?;

        Ok(Database { client })
    }
}
