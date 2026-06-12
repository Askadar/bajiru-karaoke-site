use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub twitch_id: Option<i64>,
    pub discord_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub twitch_id: Option<i64>,
    pub discord_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub twitch_id: Option<i64>,
    pub discord_id: Option<i64>,
}
