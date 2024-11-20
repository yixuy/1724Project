#[derive(Serialize, Deserialize)]
pub struct AuthorizationToken {
    pub token: String,
    pub user_id: String,
    pub is_authorized: bool,
}
