use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Capability {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCapability {
    pub title: String,
}
