use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Genre {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGenre {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGenre {
    pub title: String,
}
