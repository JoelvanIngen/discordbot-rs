mod maintenance;

use tracing::info;

use crate::{Context, Error};
pub use maintenance::{ping, version};

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
