use crate::error::DbError;
use crate::models::tag::{NewTag, Tag};
use sqlx::PgPool;

type Result<T> = std::result::Result<T, DbError>;

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Tag>> {
    sqlx::query_as::<_, Tag>("SELECT id, title FROM tags WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(DbError::from)
}

pub async fn list(pool: &PgPool) -> Result<Vec<Tag>> {
    sqlx::query_as::<_, Tag>("SELECT id, title FROM tags ORDER BY title")
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
}

/// Inserts a tag, returning the existing row if the title already exists.
pub async fn get_or_create(pool: &PgPool, new: &NewTag) -> Result<Tag> {
    sqlx::query_as::<_, Tag>(
        "INSERT INTO tags (title) VALUES ($1) \
         ON CONFLICT (title) DO UPDATE SET title = EXCLUDED.title \
         RETURNING id, title",
    )
    .bind(&new.title)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool> {
    sqlx::query("DELETE FROM tags WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}
