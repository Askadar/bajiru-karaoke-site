use crate::error::DbError;
use crate::models::song::{NewSong, Song, UpdateSong};
use sqlx::PgPool;

type Result<T> = std::result::Result<T, DbError>;

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Song>> {
    sqlx::query_as::<_, Song>(
        "SELECT id, title, art_id, audio_id, video_id, uploader_id, \
                lyrics, stream_date, play_count, duration, date_added, memo \
         FROM songs WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn list(pool: &PgPool) -> Result<Vec<Song>> {
    sqlx::query_as::<_, Song>(
        "SELECT id, title, art_id, audio_id, video_id, uploader_id, \
                lyrics, stream_date, play_count, duration, date_added, memo \
         FROM songs ORDER BY date_added DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(DbError::from)
}

pub async fn create(pool: &PgPool, new: &NewSong) -> Result<Song> {
    sqlx::query_as::<_, Song>(
        "INSERT INTO songs \
            (title, art_id, audio_id, video_id, uploader_id, lyrics, duration, memo) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
         RETURNING id, title, art_id, audio_id, video_id, uploader_id, \
                   lyrics, stream_date, play_count, duration, date_added, memo",
    )
    .bind(&new.title)
    .bind(new.art_id)
    .bind(new.audio_id)
    .bind(new.video_id)
    .bind(new.uploader_id)
    .bind(&new.lyrics)
    .bind(new.duration)
    .bind(&new.memo)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}

pub async fn update(pool: &PgPool, id: i32, upd: &UpdateSong) -> Result<Option<Song>> {
    sqlx::query_as::<_, Song>(
        "UPDATE songs \
         SET title = $1, art_id = $2, audio_id = $3, video_id = $4, \
             lyrics = $5, stream_date = $6, duration = $7, memo = $8 \
         WHERE id = $9 \
         RETURNING id, title, art_id, audio_id, video_id, uploader_id, \
                   lyrics, stream_date, play_count, duration, date_added, memo",
    )
    .bind(&upd.title)
    .bind(upd.art_id)
    .bind(upd.audio_id)
    .bind(upd.video_id)
    .bind(&upd.lyrics)
    .bind(upd.stream_date)
    .bind(upd.duration)
    .bind(&upd.memo)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

/// Increments play_count and stamps stream_date to the current time.
pub async fn increment_play_count(pool: &PgPool, id: i32) -> Result<()> {
    sqlx::query("UPDATE songs SET play_count = play_count + 1, stream_date = NOW() WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(DbError::from)
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool> {
    sqlx::query("DELETE FROM songs WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}

pub async fn get_singer_ids(pool: &PgPool, song_id: i32) -> Result<Vec<i32>> {
    sqlx::query_scalar::<_, i32>("SELECT artist_id FROM song_singers WHERE song_id = $1")
        .bind(song_id)
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
}

/// Replaces the full singer set for the given song within a transaction.
pub async fn set_singers(pool: &PgPool, song_id: i32, artist_ids: &[i32]) -> Result<()> {
    let mut tx = pool.begin().await.map_err(DbError::from)?;
    sqlx::query("DELETE FROM song_singers WHERE song_id = $1")
        .bind(song_id)
        .execute(&mut *tx)
        .await
        .map_err(DbError::from)?;
    for &artist_id in artist_ids {
        sqlx::query(
            "INSERT INTO song_singers (song_id, artist_id) \
             VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(song_id)
        .bind(artist_id)
        .execute(&mut *tx)
        .await
        .map_err(DbError::from)?;
    }
    tx.commit().await.map_err(DbError::from)
}

pub async fn get_original_artist_ids(pool: &PgPool, song_id: i32) -> Result<Vec<i32>> {
    sqlx::query_scalar::<_, i32>("SELECT artist_id FROM song_original_artists WHERE song_id = $1")
        .bind(song_id)
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
}

/// Replaces the full original artist set for the given song within a transaction.
pub async fn set_original_artists(pool: &PgPool, song_id: i32, artist_ids: &[i32]) -> Result<()> {
    let mut tx = pool.begin().await.map_err(DbError::from)?;
    sqlx::query("DELETE FROM song_original_artists WHERE song_id = $1")
        .bind(song_id)
        .execute(&mut *tx)
        .await
        .map_err(DbError::from)?;
    for &artist_id in artist_ids {
        sqlx::query(
            "INSERT INTO song_original_artists (song_id, artist_id) \
             VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(song_id)
        .bind(artist_id)
        .execute(&mut *tx)
        .await
        .map_err(DbError::from)?;
    }
    tx.commit().await.map_err(DbError::from)
}

pub async fn get_genre_ids(pool: &PgPool, song_id: i32) -> Result<Vec<i32>> {
    sqlx::query_scalar::<_, i32>("SELECT genre_id FROM song_genres WHERE song_id = $1")
        .bind(song_id)
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
}

/// Replaces the full genre set for the given song within a transaction.
pub async fn set_genres(pool: &PgPool, song_id: i32, genre_ids: &[i32]) -> Result<()> {
    let mut tx = pool.begin().await.map_err(DbError::from)?;
    sqlx::query("DELETE FROM song_genres WHERE song_id = $1")
        .bind(song_id)
        .execute(&mut *tx)
        .await
        .map_err(DbError::from)?;
    for &genre_id in genre_ids {
        sqlx::query(
            "INSERT INTO song_genres (song_id, genre_id) \
             VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(song_id)
        .bind(genre_id)
        .execute(&mut *tx)
        .await
        .map_err(DbError::from)?;
    }
    tx.commit().await.map_err(DbError::from)
}

pub async fn get_tag_ids(pool: &PgPool, song_id: i32) -> Result<Vec<i32>> {
    sqlx::query_scalar::<_, i32>("SELECT tag_id FROM song_tags WHERE song_id = $1")
        .bind(song_id)
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
}

/// Replaces the full tag set for the given song within a transaction.
pub async fn set_tags(pool: &PgPool, song_id: i32, tag_ids: &[i32]) -> Result<()> {
    let mut tx = pool.begin().await.map_err(DbError::from)?;
    sqlx::query("DELETE FROM song_tags WHERE song_id = $1")
        .bind(song_id)
        .execute(&mut *tx)
        .await
        .map_err(DbError::from)?;
    for &tag_id in tag_ids {
        sqlx::query(
            "INSERT INTO song_tags (song_id, tag_id) \
             VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(song_id)
        .bind(tag_id)
        .execute(&mut *tx)
        .await
        .map_err(DbError::from)?;
    }
    tx.commit().await.map_err(DbError::from)
}
