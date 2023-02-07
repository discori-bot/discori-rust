use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("s")]
async fn stats(_ctx: &Context, _msg: &Message) -> CommandResult {
    Ok(()) // To be modified
}

pub async fn stats_m(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(()) // To be modified
}
