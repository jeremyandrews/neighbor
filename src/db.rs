/// Functions to help with managing the backend database.
///
/// @TODO: explore making this database agnostic, supporting at least
/// MySQL and SQLite in addition to Postgres.
use sqlx::postgres::PgPool;

/// Connect to the database.
pub async fn connect() -> anyhow::Result<PgPool> {
    // @TODO useful error handling
    Ok(PgPool::connect(&dotenv::var("DATABASE_URL")?).await?)
}
