mod command;

use std::env;

use dotenvy::dotenv;
use serenity::prelude::*;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::command::{command_check, ping, pre_command, version};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load env vars
    dotenv().expect("Expected a .env file");

    let commands = vec![ping(), version()];

    let options = poise::FrameworkOptions {
        commands: commands,
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(
                env::var("PREFIX")
                    .expect("Expected a PREFIX in the environment")
                    .into(),
            ),
            ignore_bots: false, // Allow in-game chat via other bot to execute commands
            ..Default::default()
        },
        pre_command: |ctx| Box::pin(pre_command(ctx)),
        command_check: Some(|ctx| Box::pin(command_check(ctx))),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    // Login
    let token = env::var("BOT_TOKEN").expect("Expected a BOT_TOKEN in the environment");

    // Intents
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    client.start().await.expect("Error starting client");
}
