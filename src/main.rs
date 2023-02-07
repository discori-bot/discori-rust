mod commands;

use std::collections::HashSet;
use std::env;
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::macros::hook;
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use std::env;
use tracing::{debug, error, info};

use crate::commands::explore::*;
use crate::commands::home::*;
use crate::commands::invite::*;
use crate::commands::learn::*;
use crate::commands::ping::*;
use crate::commands::stats::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
}

#[group]
#[commands(explore, home, learn, stats, invite, ping)]
struct General;

#[hook]
async fn before_hook(_ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
    info!("#[command] {}: {}", msg.author.tag(), cmd_name);
    debug!(
        "#[command] {}{}: {} {}",
        msg.author.tag(),
        msg.author,
        cmd_name,
        msg.content
    );
    true
}
struct Handler;

#[tokio::main]
async fn main() {
    // Make a .env file containing your bot token and invite link in the top-level directory
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let bot_id: u64 = env::var("BOT_ID")
        .expect("Expected a bot id in the environment")
        .parse()
        .expect("Expected bot id to be a number");
    let owner_id: u64 = env::var("OWNER_ID")
        .expect("Expected an owner id in the environment")
        .parse()
        .expect("Expected owner id to be a number");
    let owners = vec![UserId(owner_id)].into_iter().collect();

    let framework = StandardFramework::new()
        .before(before_hook)
        .configure(|c| c.on_mention(Some(bot_id.into())).owners(owners).prefix("["))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }
}
