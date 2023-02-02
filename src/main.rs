mod commands;

use std::collections::HashSet;
use std::env;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::prelude::*;
use tracing::{error};

use crate::commands::explore::*;
use crate::commands::home::*;
use crate::commands::learn::*;
use crate::commands::stats::*;


#[group]
#[commands(explore, home, learn, stats)]
struct General;

#[tokio::main]
async fn main() {
    // Make a .env file containing your bot token in the top-level directory
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new(&token);

    // Fetch owner and bot data
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(e) => panic!("Could not access application info: {:?}", e),
    };

     let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix("[")).group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }
}