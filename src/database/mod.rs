use sqlx::SqlitePool;

use crate::Error;

pub mod autoreply;

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}
