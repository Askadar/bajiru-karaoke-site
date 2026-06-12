use crate::error::DbError;
use crate::models::user::{NewUser, UpdateUser, User};
use sqlx::PgPool;

type Result<T> = std::result::Result<T, DbError>;

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<User>> {
    sqlx::query_as::<_, User>("SELECT id, username, twitch_id, discord_id FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(DbError::from)
}

pub async fn get_by_twitch_id(pool: &PgPool, twitch_id: i64) -> Result<Option<User>> {
    sqlx::query_as::<_, User>(
        "SELECT id, username, twitch_id, discord_id FROM users WHERE twitch_id = $1",
    )
    .bind(twitch_id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn get_by_discord_id(pool: &PgPool, discord_id: i64) -> Result<Option<User>> {
    sqlx::query_as::<_, User>(
        "SELECT id, username, twitch_id, discord_id FROM users WHERE discord_id = $1",
    )
    .bind(discord_id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Option<User>> {
    sqlx::query_as::<_, User>(
        "SELECT id, username, twitch_id, discord_id FROM users WHERE username = $1",
    )
    .bind(username)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn list(pool: &PgPool) -> Result<Vec<User>> {
    sqlx::query_as::<_, User>("SELECT id, username, twitch_id, discord_id FROM users ORDER BY id")
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
}

pub async fn create(pool: &PgPool, new: &NewUser) -> Result<User> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, twitch_id, discord_id) \
         VALUES ($1, $2, $3) \
         RETURNING id, username, twitch_id, discord_id",
    )
    .bind(&new.username)
    .bind(new.twitch_id)
    .bind(new.discord_id)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}

/// Inserts or updates by `twitch_id`, refreshing the username on conflict.
pub async fn upsert_by_twitch(pool: &PgPool, new: &NewUser) -> Result<User> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, twitch_id, discord_id) \
         VALUES ($1, $2, $3) \
         ON CONFLICT (twitch_id) DO UPDATE SET username = EXCLUDED.username \
         RETURNING id, username, twitch_id, discord_id",
    )
    .bind(&new.username)
    .bind(new.twitch_id)
    .bind(new.discord_id)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}

/// Inserts or updates by `discord_id`, refreshing the username on conflict.
pub async fn upsert_by_discord(pool: &PgPool, new: &NewUser) -> Result<User> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (username, twitch_id, discord_id) \
         VALUES ($1, $2, $3) \
         ON CONFLICT (discord_id) DO UPDATE SET username = EXCLUDED.username \
         RETURNING id, username, twitch_id, discord_id",
    )
    .bind(&new.username)
    .bind(new.twitch_id)
    .bind(new.discord_id)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}

pub async fn update(pool: &PgPool, id: i32, upd: &UpdateUser) -> Result<Option<User>> {
    sqlx::query_as::<_, User>(
        "UPDATE users \
         SET username = $1, twitch_id = $2, discord_id = $3 \
         WHERE id = $4 \
         RETURNING id, username, twitch_id, discord_id",
    )
    .bind(&upd.username)
    .bind(upd.twitch_id)
    .bind(upd.discord_id)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}

pub async fn get_favorite_song_ids(pool: &PgPool, user_id: i32) -> Result<Vec<i32>> {
    sqlx::query_scalar::<_, i32>("SELECT song_id FROM user_favorite_songs WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
}

pub async fn add_favorite_song(pool: &PgPool, user_id: i32, song_id: i32) -> Result<()> {
    sqlx::query(
        "INSERT INTO user_favorite_songs (user_id, song_id) \
         VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(song_id)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(DbError::from)
}

pub async fn remove_favorite_song(pool: &PgPool, user_id: i32, song_id: i32) -> Result<()> {
    sqlx::query("DELETE FROM user_favorite_songs WHERE user_id = $1 AND song_id = $2")
        .bind(user_id)
        .bind(song_id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(DbError::from)
}
