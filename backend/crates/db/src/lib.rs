pub mod error;
pub mod models;
pub mod queries;

pub use sqlx::PgPool;

pub type Result<T> = std::result::Result<T, error::DbError>;

pub async fn connect(database_url: &str) -> Result<PgPool> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .map_err(error::DbError::from)
}
