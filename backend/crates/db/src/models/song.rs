use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub art_id: Option<i32>,
    pub audio_id: Option<i32>,
    pub video_id: Option<i32>,
    pub uploader_id: Option<i32>,
    pub lyrics: Option<String>,
    pub stream_date: Option<DateTime<Utc>>,
    pub play_count: i32,
    /// Duration in milliseconds.
    pub duration: i32,
    pub date_added: DateTime<Utc>,
    /// Display for admins on queue page.
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSong {
    pub title: String,
    pub art_id: Option<i32>,
    pub audio_id: Option<i32>,
    pub video_id: Option<i32>,
    pub uploader_id: Option<i32>,
    pub lyrics: Option<String>,
    pub duration: i32,
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSong {
    pub title: String,
    pub art_id: Option<i32>,
    pub audio_id: Option<i32>,
    pub video_id: Option<i32>,
    pub lyrics: Option<String>,
    pub stream_date: Option<DateTime<Utc>>,
    pub duration: i32,
    pub memo: Option<String>,
}
