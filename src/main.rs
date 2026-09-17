mod command;
mod database;

use std::{env, str::FromStr};

use dotenvy::dotenv;
use serenity::prelude::*;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::command::{autoreply, command_check, ping, pre_command, version};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub db_pool: sqlx::SqlitePool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load env vars
    dotenv().expect("Expected a .env file");

    let commands = vec![ping(), version(), autoreply()];

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
        on_error: |err| Box::pin(command::on_error(err)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                info!("Logged in as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let db_url =
                    env::var("SQLITE_URL").expect("Expected a SQLITE_URL in the environment");
                let conn_options = SqliteConnectOptions::from_str(&db_url)?.create_if_missing(true);

                let pool = SqlitePoolOptions::new().connect_with(conn_options).await?;

                database::init_db(&pool).await?;

                Ok(Data { db_pool: pool })
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
