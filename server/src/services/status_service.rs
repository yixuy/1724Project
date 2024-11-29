// use crate::db::operations;
// use crate::error::DatabaseError;

// /* User status-related logics */

// pub async fn set_online(username: &str) -> Result<(), DatabaseError> {
//     // marks a user as online
//     operations::update_user_status(username, true).await?;
//     Ok(())
// }

// pub async fn set_offline(username: &str) -> Result<(), DatabaseError> {
//     // marks a user as offline
//     operations::update_user_status(username, false).await?;
//     Ok(())
// }

// pub async fn fetch_status(username: &str) -> Result<String, DatabaseError> {
//     // retrieves online/offline status for users
//     let status = operations::get_user_status(username).await?;
//     Ok(status)
// }