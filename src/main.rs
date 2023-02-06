mod commands;

use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::macros::hook;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::collections::HashSet;
use std::env;
use tracing::{debug, error, info};

use crate::commands::explore::*;
use crate::commands::home::*;
use crate::commands::learn::*;
use crate::commands::stats::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
}

#[group]
#[commands(explore, home, learn, stats)]
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

#[tokio::main]
async fn main() {
    // Make a .env file containing your bot token in the top-level directory
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new(&token);

    // Fetch owner and bot data
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(e) => panic!("Could not access application info: {e:?}"),
    };

    let framework = StandardFramework::new()
        .before(before_hook)
        .configure(|c| {
            c.on_mention(Some(u64::from(bot_id).into()))
                .owners(owners)
                .prefix("[")
        })
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }
}
