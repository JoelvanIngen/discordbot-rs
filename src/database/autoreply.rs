use sqlx::{SqlitePool, query, query_scalar};

use sqlx::Error;

/// Adds an autoreply entry to database
/// Autoreply entry must already be checked for validity
pub async fn add_reply(pool: &SqlitePool, trigger: &String, reply: &String) -> Result<(), Error> {
    query("INSERT INTO autoreplies (trigger, reply) VALUES (?, ?)")
        .bind(trigger)
        .bind(reply)
        .execute(pool)
        .await?;

    Ok(())
}

/// Retrieves one random reply to a specific trigger
/// Returns None if trigger does not exist
pub async fn get_reply_random(
    pool: &SqlitePool,
    trigger: &String,
) -> Result<Option<String>, Error> {
    query_scalar("SELECT reply FROM autoreplies WHERE trigger = ? ORDER BY RANDOM() LIMIT 1")
        .bind(trigger)
        .fetch_optional(pool)
        .await
}

/// Attempts to delete a trigger, value pair
pub async fn delete_reply(
    pool: &SqlitePool,
    trigger: &String,
    reply: &String,
) -> Result<(), Error> {
    query("DELETE FROM autoreplies WHERE trigger = ? AND reply = ?")
        .bind(trigger)
        .bind(reply)
        .execute(pool)
        .await?;

    Ok(())
}

/// Attempts to delete a trigger and all its replies
pub async fn delete_trigger(pool: &SqlitePool, trigger: &String) -> Result<(), Error> {
    query("DELETE FROM autoreplies WHERE trigger = ?")
        .bind(trigger)
        .execute(pool)
        .await?;

    Ok(())
}

/// Lists all triggers, not their replies
pub async fn get_triggers(pool: &SqlitePool) -> Result<Vec<String>, Error> {
    query_scalar("SELECT DISTINCT trigger FROM autoreplies LIMIT 100")
        .fetch_all(pool)
        .await
}

/// Lists all replies of specific trigger
pub async fn get_replies(pool: &SqlitePool, trigger: &String) -> Result<Vec<String>, Error> {
    query_scalar("SELECT DISTINCT reply FROM autoreplies WHERE trigger = ? LIMIT 100")
        .bind(trigger)
        .fetch_all(pool)
        .await
}
