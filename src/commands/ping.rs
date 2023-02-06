use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Instant;

#[command]
#[aliases("latency")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let before = Instant::now();
    let mut m = msg.channel_id.say(&ctx.http, "Pong!").await?;
    let after = Instant::now();

    m.edit(ctx, |m| m.content(format!("Pong!\nMy ping is {}ms", (after - before).as_millis()))).await?;

    Ok(()) // To be modified
}
