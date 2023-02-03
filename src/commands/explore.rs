use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("e")]
async fn explore(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(()) // To be modified
}
