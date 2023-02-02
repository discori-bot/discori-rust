use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("s")]
async fn search(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(()) // To be modified
}
