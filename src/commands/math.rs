use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    println!("{} {}", first, second);

    let result = first + second;
    if let Err(e) = msg.channel_id.say(&ctx.http, &result.to_string()).await {
        println!("Error in command add: {:?}", e);
    }

    Ok(())
}
