use sqlx::SqlitePool;

use crate::Error;

pub mod autoreply;

pub async fn init_db(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::raw_sql(
        r#"
        CREATE TABLE IF NOT EXISTS autoreplies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            trigger TEXT NOT NULL,
            reply TEXT NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS idx_autoreplies_trigger ON autoreplies(trigger, reply);
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
