mod commands;

use std::collections::HashSet;
use std::{fs, sync::Arc};

use log::*;
use simplelog::*;

use tokio_postgres::NoTls;

use serenity::prelude::*;
use serenity::{
    async_trait,
    framework::standard::{
        Args, CommandResult, CommandGroup,
        HelpOptions, help_commands, StandardFramework,
        macros::{group, help, hook},
    },
    http::Http,
    model::{
        channel::{Message},
        gateway::Ready,
        id::UserId,
    },
};

use commands::directory::*;
use saddleback_cs_bot::DbConnection;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }
}

#[group]
#[commands(name)]
struct General;

#[group]
#[prefix = "course"]
#[commands(add, remove)]
struct Course;

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

#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    let name = format!("{}#{}", msg.author.name, msg.author.discriminator);
    info!("Executing `{}` for {} in {}", command_name, name, msg.channel_id.name(&ctx.cache).await.unwrap());

    true
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    if let Err(e) = command_result {
        error!("Command {} returned error: {:?}", command_name, e);
    }
}

#[tokio::main]
async fn main() {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap();
    info!("What is this, like, a Minecraft server-esque power grab or something?");

    let token = fs::read_to_string("token")
        .expect("Could not read token file");

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
        .before(before)
        .after(after)
        .help(&HELP)
        .group(&GENERAL_GROUP)
        .group(&COURSE_GROUP);

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    match tokio_postgres::connect("host=localhost user=saddlebot dbname=saddlebot", NoTls).await {
        Ok((cl, co)) => {
            tokio::spawn(async move {
                if let Err(e) = co.await {
                    warn!("Error connecting to database: {:?}", e);
                }
            });

            {
                let mut data = client.data.write().await;
                data.insert::<DbConnection>(Arc::new(Mutex::new(cl)));
            }
        },
        Err(e) => {
            warn!("Error connecting to database: {:?}", e);
        }
    };


    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }
}
