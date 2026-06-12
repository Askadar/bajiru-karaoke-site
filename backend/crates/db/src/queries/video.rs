use crate::error::DbError;
use crate::models::video::{NewVideo, Video};
use sqlx::PgPool;

type Result<T> = std::result::Result<T, DbError>;

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Video>> {
    sqlx::query_as::<_, Video>("SELECT id, external_url FROM video WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(DbError::from)
}

pub async fn create(pool: &PgPool, new: &NewVideo) -> Result<Video> {
    sqlx::query_as::<_, Video>(
        "INSERT INTO video (external_url) VALUES ($1) RETURNING id, external_url",
    )
    .bind(&new.external_url)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool> {
    sqlx::query("DELETE FROM video WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}
