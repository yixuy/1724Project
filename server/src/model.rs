use crate::schema::*;
use serde::{Deserialize, Serialize};
pub mod user;

// #[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
// pub struct User {
//     pub id: String,
//     pub username: String,
//     pub password: String,
//     #[derivative(Default(value = "chrono::offset::Utc::now()"))]
//     pub created_at: DateTime<Utc>,
//     pub is_authorized: bool,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
// impl User {
//     pub async fn find_by_id(
//         conn: &mut SqliteConnection,
//         user_id: &str,
//     ) -> Result<Option<User>, diesel::result::Error> {
//         use crate::schema::users::dsl::*;
//         let user = users
//             .filter(id.eq(user_id))
//             .first::<User>(conn)
//             .optional()?;
//         Ok(user)
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
// pub struct Room {
//     pub id: String,
//     pub name: String,
//     pub last_message: String,
//     pub participant_ids: String,
//     pub created_at: String,
// }
