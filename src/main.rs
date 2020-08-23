mod commands;

use std::collections::HashSet;
use std::{fs, sync::Arc};

use tokio_postgres::NoTls;

use serenity::prelude::*;
use serenity::{
    async_trait,
    framework::standard::{
        Args, CommandResult, CommandGroup,
        HelpOptions, help_commands, StandardFramework,
        macros::{group, help},
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
        println!("Connected as {}", ready.user.name);
    }
}

#[group]
#[commands(name, course)]
struct General;

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
        .help(&HELP)
        .group(&GENERAL_GROUP);

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    match tokio_postgres::connect("host=localhost user=saddlebot dbname=saddlebot", NoTls).await {
        Ok((cl, co)) => {
            tokio::spawn(async move {
                if let Err(e) = co.await {
                    println!("Error connecting to database: {:?}", e);
                }
            });

            {
                let mut data = client.data.write().await;
                data.insert::<DbConnection>(Arc::new(Mutex::new(cl)));
            }
        },
        Err(e) => {
            println!("Error connecting to database: {:?}", e);
        }
    };


    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
