use std::fmt::{Display, Write};

use crate::{Context, Error, database};

#[poise::command(prefix_command, slash_command, subcommands("add", "remove", "list"))]
pub async fn autoreply(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply(format!(
        "Hi! Use one of the subcommands (add, remove, list) after the {}autoreply command!",
        ctx.prefix()
    ))
    .await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Trigger word"] trigger: String,
    #[description = "Response"] reply: String,
) -> Result<(), Error> {
    if trigger.contains(' ') {
        ctx.reply("Error: command cannot contains spaces").await?;
        return Ok(());
    }

    database::autoreply::add_reply(&ctx.data().db_pool, &trigger, &reply).await?;

    ctx.reply(format!("Successfully saved {} => {}", trigger, reply))
        .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Trigger word"] trigger: String,
    #[description = "Response"] reply: Option<String>,
) -> Result<(), Error> {
    match reply {
        Some(r) => {
            match database::autoreply::delete_reply(&ctx.data().db_pool, &trigger, &r).await? {
                true => {
                    ctx.reply(format!("Successfully deleted reply {} => {}", trigger, r))
                        .await?;
                }
                false => {
                    ctx.reply(format!("No command-reply found for {} => {}", trigger, r))
                        .await?;
                }
            }
        }
        None => match database::autoreply::delete_trigger(&ctx.data().db_pool, &trigger).await? {
            true => {
                ctx.reply(format!("Successfully deleted command {}", trigger))
                    .await?;
            }
            false => {
                ctx.reply(format!("No command found for {}", trigger))
                    .await?;
            }
        },
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Trigger word"] trigger: Option<String>,
) -> Result<(), Error> {
    match trigger {
        Some(t) => {
            let replies = database::autoreply::get_replies(&ctx.data().db_pool, &t).await?;
            if replies.is_empty() {
                ctx.say("No replies registered!").await?;
            } else {
                ctx.say(format_list("Registered replies:", &replies))
                    .await?;
            }
        }
        None => {
            let triggers = database::autoreply::get_triggers(&ctx.data().db_pool).await?;
            if triggers.is_empty() {
                ctx.say("No commands registered!").await?;
            } else {
                ctx.say(format_list("Registered commands:", &triggers))
                    .await?;
            }
        }
    }
    Ok(())
}

/// Formats list to print to user
fn format_list<T: Display>(prefix: &str, items: &[T]) -> String {
    let mut res = prefix.to_string();
    for item in items {
        write!(res, "\n • {item}").unwrap();
    }
    res
}
