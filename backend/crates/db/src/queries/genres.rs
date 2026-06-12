use crate::error::DbError;
use crate::models::genre::{Genre, NewGenre, UpdateGenre};
use sqlx::PgPool;

type Result<T> = std::result::Result<T, DbError>;

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Genre>> {
    sqlx::query_as::<_, Genre>("SELECT id, title FROM genres WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(DbError::from)
}

pub async fn list(pool: &PgPool) -> Result<Vec<Genre>> {
    sqlx::query_as::<_, Genre>("SELECT id, title FROM genres ORDER BY title")
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
}

pub async fn create(pool: &PgPool, new: &NewGenre) -> Result<Genre> {
    sqlx::query_as::<_, Genre>(
        "INSERT INTO genres (title) VALUES ($1) \
         ON CONFLICT (title) DO UPDATE SET title = EXCLUDED.title \
         RETURNING id, title",
    )
    .bind(&new.title)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}

pub async fn update(pool: &PgPool, id: i32, upd: &UpdateGenre) -> Result<Option<Genre>> {
    sqlx::query_as::<_, Genre>("UPDATE genres SET title = $1 WHERE id = $2 RETURNING id, title")
        .bind(&upd.title)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(DbError::from)
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool> {
    sqlx::query("DELETE FROM genres WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}
