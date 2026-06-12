use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct CoverArt {
    pub id: i32,
    pub file_path: String,
    pub thumbnail: Option<String>,
    pub credits: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCoverArt {
    pub file_path: String,
    pub thumbnail: Option<String>,
    pub credits: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCoverArt {
    pub file_path: String,
    pub thumbnail: Option<String>,
    pub credits: Option<String>,
}
