use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Video {
    pub id: i32,
    pub external_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewVideo {
    pub external_url: String,
}
