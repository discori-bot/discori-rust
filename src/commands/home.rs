use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("h")]
async fn home(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(()) // To be modified
}

pub async fn home_m(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(()) // To be modified
}
