use crate::{Context, Error};

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
