use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("e")]
async fn explore(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(()) // To be modified
}
