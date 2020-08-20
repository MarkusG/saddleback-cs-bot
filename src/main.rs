use std::{collections::{HashSet}, env};

use serenity::prelude::*;
use serenity::{
    async_trait,
    framework::standard::{
        Args, CommandResult, CommandGroup,
        HelpOptions, help_commands, StandardFramework,
        macros::{command, group, help},
    },
    http::Http,
    model::{
        channel::{Message},
        gateway::Ready,
        id::UserId,
    },
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }
}

#[group]
#[prefix = "math"]
#[commands(add)]
struct Math;

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("What is this, like, a Minecraft server-esque power grab or something?");

    let token = env::var("SADDLEBACK_CS_BOT_TOKEN")
        .expect("SADDLEBACK_CS_BOT_TOKEN not set");

    let http = Http::new_with_token(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(e) => panic!("Error getting application info: {:?}", e)
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                   .with_whitespace(true)
                   .on_mention(Some(bot_id))
                   .prefix("!")
                   .owners(owners))
        .help(&HELP)
        .group(&MATH_GROUP);

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}

#[command]
async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    println!("{} {}", first, second);

    let result = first + second;
    if let Err(e) = msg.channel_id.say(&ctx.http, &result.to_string()).await {
        println!("Error in command add: {:?}", e);
    }

    Ok(())
}
