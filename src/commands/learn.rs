use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("l")]
async fn learn(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(()) // To be modified
}