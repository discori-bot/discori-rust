use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[aliases("e")]
async fn explore(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Hi Sherrin").await {
        eprintln!("Error sending message: {:?}", why);
    }
    Ok(()) // To be modified
}

pub async fn explore_m(ctx: &Context, msg: &Message, args: impl Iterator<Item = &str>) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Hi Sherrin").await {
        eprintln!("Error sending message: {:?}", why);
    }
    let mut flag_args = Vec::new();
    let mut normal_args = Vec::new();
    for arg in args {
        if arg.starts_with("--") {
            flag_args.push(arg);
        } else {
            normal_args.push(arg);
        }
    }
    for arg in flag_args {
        if let Err(why) = msg.channel_id.say(&ctx.http, format!("Flag argument: {}", arg)).await {
            eprintln!("Error sending message: {:?}", why);
        }
    }
    for arg in normal_args {
        if let Err(why) = msg.channel_id.say(&ctx.http, format!("NOrmal argument: {}", arg)).await {
            eprintln!("Error sending message: {:?}", why);
        }
    }
    Ok(()) // To be modified
}

