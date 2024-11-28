use uuid::Uuid;

/* User data model */

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub status: String,
}

impl User {
    fn to_db_model(self) {
        todo!();
        // converts a User struct to a format compatible with SurrealDB
    }

    fn from_db_model() -> Self {
        todo!();
        // converts SurrealDB data to a User struct
        User {
            id: Uuid::new_v4(),
            username: "".to_string(),
            hashed_password: "".to_string(),
            status: "".to_string(),
        }
    }
}