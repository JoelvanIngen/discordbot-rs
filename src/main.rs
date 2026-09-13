use std::env;

mod logging;

use dotenvy;
use log::{info, warn, error};
use log::LevelFilter;
use logging::Logger;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

static LOGGER: Logger = Logger;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let _ = log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info));
    error!("Test");
    println!("Test_print");

    // Load env vars
    dotenvy::dotenv().expect("Expected a .env file");

    // Login
    let token = env::var("BOT_TOKEN").expect("Expected a Discord token in the environment");

    // Intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    // Create client instance
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
