use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Audio {
    pub id: i32,
    pub file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAudio {
    pub file_path: String,
}
