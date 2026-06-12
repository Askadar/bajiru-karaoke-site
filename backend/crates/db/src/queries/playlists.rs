use crate::error::DbError;
use crate::models::playlist::{NewPlaylist, Playlist, UpdatePlaylist};
use sqlx::PgPool;

type Result<T> = std::result::Result<T, DbError>;

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Playlist>> {
    sqlx::query_as::<_, Playlist>(
        "SELECT id, title, description, created_by FROM playlists WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn list(pool: &PgPool) -> Result<Vec<Playlist>> {
    sqlx::query_as::<_, Playlist>(
        "SELECT id, title, description, created_by FROM playlists ORDER BY id",
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::from)
}

pub async fn list_by_user(pool: &PgPool, user_id: i32) -> Result<Vec<Playlist>> {
    sqlx::query_as::<_, Playlist>(
        "SELECT id, title, description, created_by FROM playlists \
         WHERE created_by = $1 ORDER BY id",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(DbError::from)
}

pub async fn create(pool: &PgPool, new: &NewPlaylist) -> Result<Playlist> {
    sqlx::query_as::<_, Playlist>(
        "INSERT INTO playlists (title, description, created_by) \
         VALUES ($1, $2, $3) \
         RETURNING id, title, description, created_by",
    )
    .bind(&new.title)
    .bind(&new.description)
    .bind(new.created_by)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}

pub async fn update(pool: &PgPool, id: i32, upd: &UpdatePlaylist) -> Result<Option<Playlist>> {
    sqlx::query_as::<_, Playlist>(
        "UPDATE playlists SET title = $1, description = $2 WHERE id = $3 \
         RETURNING id, title, description, created_by",
    )
    .bind(&upd.title)
    .bind(&upd.description)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn get_song_ids(pool: &PgPool, playlist_id: i32) -> Result<Vec<i32>> {
    sqlx::query_scalar::<_, i32>(
        "SELECT song_id FROM playlist_songs \
         WHERE playlist_id = $1 ORDER BY sort_order",
    )
    .bind(playlist_id)
    .fetch_all(pool)
    .await
    .map_err(DbError::from)
}

/// Replaces all songs in the playlist, assigning sequential positions.
pub async fn set_songs(pool: &PgPool, playlist_id: i32, song_ids: &[i32]) -> Result<()> {
    let mut tx = pool.begin().await.map_err(DbError::from)?;
    sqlx::query("DELETE FROM playlist_songs WHERE playlist_id = $1")
        .bind(playlist_id)
        .execute(&mut *tx)
        .await
        .map_err(DbError::from)?;
    for (pos, &song_id) in song_ids.iter().enumerate() {
        sqlx::query(
            "INSERT INTO playlist_songs (playlist_id, song_id, sort_order) \
             VALUES ($1, $2, $3)",
        )
        .bind(playlist_id)
        .bind(song_id)
        .bind(pos as i32)
        .execute(&mut *tx)
        .await
        .map_err(DbError::from)?;
    }
    tx.commit().await.map_err(DbError::from)
}

pub async fn add_song(pool: &PgPool, playlist_id: i32, song_id: i32) -> Result<()> {
    sqlx::query(
        "INSERT INTO playlist_songs (playlist_id, song_id, sort_order) \
         VALUES ($1, $2, \
             COALESCE((SELECT MAX(sort_order) + 1 FROM playlist_songs WHERE playlist_id = $1), 0) \
         ) ON CONFLICT DO NOTHING",
    )
    .bind(playlist_id)
    .bind(song_id)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(DbError::from)
}

pub async fn remove_song(pool: &PgPool, playlist_id: i32, song_id: i32) -> Result<()> {
    sqlx::query("DELETE FROM playlist_songs WHERE playlist_id = $1 AND song_id = $2")
        .bind(playlist_id)
        .bind(song_id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(DbError::from)
}
