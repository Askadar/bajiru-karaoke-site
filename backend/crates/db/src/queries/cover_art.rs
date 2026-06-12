use crate::error::DbError;
use crate::models::cover_art::{CoverArt, NewCoverArt, UpdateCoverArt};
use sqlx::PgPool;

type Result<T> = std::result::Result<T, DbError>;

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<CoverArt>> {
    sqlx::query_as::<_, CoverArt>(
        "SELECT id, file_path, thumbnail, credits FROM cover_art WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn create(pool: &PgPool, new: &NewCoverArt) -> Result<CoverArt> {
    sqlx::query_as::<_, CoverArt>(
        "INSERT INTO cover_art (file_path, thumbnail, credits) \
         VALUES ($1, $2, $3) \
         RETURNING id, file_path, thumbnail, credits",
    )
    .bind(&new.file_path)
    .bind(&new.thumbnail)
    .bind(&new.credits)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}

pub async fn update(pool: &PgPool, id: i32, upd: &UpdateCoverArt) -> Result<Option<CoverArt>> {
    sqlx::query_as::<_, CoverArt>(
        "UPDATE cover_art \
         SET file_path = $1, thumbnail = $2, credits = $3 \
         WHERE id = $4 \
         RETURNING id, file_path, thumbnail, credits",
    )
    .bind(&upd.file_path)
    .bind(&upd.thumbnail)
    .bind(&upd.credits)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool> {
    sqlx::query("DELETE FROM cover_art WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}
