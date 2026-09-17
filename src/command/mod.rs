mod autoreply;
mod maintenance;

use poise::FrameworkError;
use tracing::info;

use crate::{Context, Data, Error, database};
pub use autoreply::autoreply;
pub use maintenance::{ping, version};

pub async fn pre_command(ctx: Context<'_>) {
    info!(
        "Received command from user {}: {}",
        ctx.author().name,
        ctx.invocation_string(),
    )
}

pub async fn command_check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.author().id != ctx.cache().current_user().id)
}

pub async fn on_error(error: FrameworkError<'_, Data, Error>) {
    match error {
        FrameworkError::UnknownCommand {
            msg,
            content_start,
            framework,
            ..
        } => {
            let content_after_prefix = &msg.content[content_start as usize..];
            let trigger = match content_after_prefix.split_whitespace().next() {
                Some(cmd) => cmd.to_string(),
                None => return,
            };

            match database::autoreply::get_reply_random(
                &framework.user_data().await.db_pool,
                &trigger.into(),
            )
            .await
            {
                Ok(Some(reply)) => {
                    let ctx = framework.serenity_context;
                    if let Err(e) = msg.channel_id.say(ctx, reply).await {
                        tracing::error!("Failed to send autoreply: {e}");
                    }
                }
                Ok(None) => {
                    // Autoreply does not exist, ignore
                }
                Err(e) => {
                    tracing::error!("Database error fetching autoreply: {e}");
                }
            }
        }
        // Fall back to default error handling
        other => {
            if let Err(e) = poise::builtins::on_error(other).await {
                tracing::error!("Error in on_error handler: {e}");
            }
        }
    }
}
