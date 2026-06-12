use crate::error::DbError;
use crate::models::capability::{Capability, NewCapability};
use sqlx::PgPool;

type Result<T> = std::result::Result<T, DbError>;

pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Capability>> {
    sqlx::query_as::<_, Capability>("SELECT id, title FROM capabilities WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(DbError::from)
}

pub async fn list(pool: &PgPool) -> Result<Vec<Capability>> {
    sqlx::query_as::<_, Capability>("SELECT id, title FROM capabilities ORDER BY title")
        .fetch_all(pool)
        .await
        .map_err(DbError::from)
}

pub async fn list_for_user(pool: &PgPool, user_id: i32) -> Result<Vec<Capability>> {
    sqlx::query_as::<_, Capability>(
        "SELECT c.id, c.title \
         FROM capabilities c \
         JOIN user_capabilities uc ON uc.capability_id = c.id \
         WHERE uc.user_id = $1 \
         ORDER BY c.title",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(DbError::from)
}

/// Creates the capability, returning the existing row if the title already exists.
pub async fn create(pool: &PgPool, new: &NewCapability) -> Result<Capability> {
    sqlx::query_as::<_, Capability>(
        "INSERT INTO capabilities (title) VALUES ($1) \
         ON CONFLICT (title) DO UPDATE SET title = EXCLUDED.title \
         RETURNING id, title",
    )
    .bind(&new.title)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool> {
    sqlx::query("DELETE FROM capabilities WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .map_err(DbError::from)
}

pub async fn add_to_user(pool: &PgPool, user_id: i32, capability_id: i32) -> Result<()> {
    sqlx::query(
        "INSERT INTO user_capabilities (user_id, capability_id) \
         VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(capability_id)
    .execute(pool)
    .await
    .map(|_| ())
    .map_err(DbError::from)
}

pub async fn remove_from_user(pool: &PgPool, user_id: i32, capability_id: i32) -> Result<()> {
    sqlx::query("DELETE FROM user_capabilities WHERE user_id = $1 AND capability_id = $2")
        .bind(user_id)
        .bind(capability_id)
        .execute(pool)
        .await
        .map(|_| ())
        .map_err(DbError::from)
}

pub async fn user_has(pool: &PgPool, user_id: i32, capability_id: i32) -> Result<bool> {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(\
             SELECT 1 FROM user_capabilities \
             WHERE user_id = $1 AND capability_id = $2\
         )",
    )
    .bind(user_id)
    .bind(capability_id)
    .fetch_one(pool)
    .await
    .map_err(DbError::from)
}
