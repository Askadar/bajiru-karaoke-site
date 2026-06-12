use crate::error::DbError;
use crate::models::audio::{Audio, NewAudio};
use sqlx::PgPool;

type Result<T> = std::result::Result<T, DbError>;

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Audio>> {
    sqlx::query_as::<_, Audio>("SELECT id, file_path FROM audio WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(DbError::from)
}

pub async fn create(pool: &PgPool, new: &NewAudio) -> Result<Audio> {
    sqlx::query_as::<_, Audio>("INSERT INTO audio (file_path) VALUES ($1) RETURNING id, file_path")
        .bind(&new.file_path)
        .fetch_one(pool)
        .await
        .map_err(DbError::from)
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool> {
    sqlx::query("DELETE FROM audio WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}
