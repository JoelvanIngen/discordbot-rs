use tracing::info;

use crate::{Context, Error};

pub async fn pre_command(ctx: Context<'_>) {
    info!(
        "Received command from user {}: {}",
        ctx.author().name,
        ctx.invocation_string(),
    )
}

pub async fn command_check(ctx: Context<'_>) -> Result<bool, Error> {
    return Ok(ctx.author().id != ctx.cache().current_user().id);
}

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn version(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply(format!("My version is {}", env!("CARGO_PKG_VERSION")))
        .await?;
    Ok(())
}
