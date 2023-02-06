use std::env;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    let invite = env::var("INVITE_LINK").expect("Expected an invite link in the environment");

    if let Err(e) = msg.channel_id.say(&ctx.http, format!("Here's my invite link: \n {}", invite)).await {
        eprintln!("Sending message failed: {:?}", e);
    }
    Ok(())
}
