use crate::error::DbError;
use crate::models::artist::{Artist, NewArtist, UpdateArtist};
use sqlx::PgPool;

type Result<T> = std::result::Result<T, DbError>;

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Artist>> {
    sqlx::query_as::<_, Artist>("SELECT id, title, description FROM artists WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(DbError::from)
}

pub async fn list(pool: &PgPool) -> Result<Vec<Artist>> {
    sqlx::query_as::<_, Artist>("SELECT id, title, description FROM artists ORDER BY title")
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
}

pub async fn create(pool: &PgPool, new: &NewArtist) -> Result<Artist> {
    sqlx::query_as::<_, Artist>(
        "INSERT INTO artists (title, description) \
         VALUES ($1, $2) \
         RETURNING id, title, description",
    )
    .bind(&new.title)
    .bind(&new.description)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}

pub async fn update(pool: &PgPool, id: i32, upd: &UpdateArtist) -> Result<Option<Artist>> {
    sqlx::query_as::<_, Artist>(
        "UPDATE artists \
         SET title = $1, description = $2 \
         WHERE id = $3 \
         RETURNING id, title, description",
    )
    .bind(&upd.title)
    .bind(&upd.description)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(DbError::from)
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool> {
    sqlx::query("DELETE FROM artists WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}
