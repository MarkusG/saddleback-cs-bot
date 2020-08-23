use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

use saddleback_cs_bot::DbConnection;

#[command]
pub async fn name(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let response;
    let first = args.single::<String>()?;
    if let Ok(s) = args.single::<String>() {
        response = format!("Hello, {} {}!", first, s);
    } else {
        response = format!("Hello, {}!", first);
    }

    // get the db connection from context
    let db_client_lock = ctx.data.read().await.get::<DbConnection>().cloned().unwrap();
    // wait for it to be available and lock it for our use
    let db_client = db_client_lock.lock().await;
    // ensure our connection is present and run our query
    if let Some(c) = &*db_client {
        let rows = c
            .query("SELECT * from member", &[])
            .await;
        match rows {
            Ok(r) => {
                for row in r {
                    let first_name: &str = row.get(2);
                    let last_name: &str = row.get(3);
                    println!("{} {}", first_name, last_name);
                }
            },
            Err(e) => println!("Error executing query: {:?}", e)
        }
    }


    // worry about casing in postgres

    if let Err(e) = msg.channel_id.say(&ctx.http, response).await {
        println!("Error in command add: {:?}", e);
    }

    Ok(())
}

#[command]
pub async fn course(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<f64>()?;

    if let Err(e) = msg.channel_id.say(&ctx.http, &first).await {
        println!("Error in command add: {:?}", e);
    }

    Ok(())
}
