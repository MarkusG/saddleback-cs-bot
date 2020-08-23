use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

use saddleback_cs_bot::DbConnection;

#[command]
pub async fn name(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<String>()?;
    let last = args.single::<String>()?;

    // get the db connection from context
    let db_client_lock = ctx.data.read().await.get::<DbConnection>().cloned().unwrap();
    // wait for it to be available and lock it for our use
    let db_client = db_client_lock.lock().await;
    // ensure our connection is present and run our query
    if let Some(c) = &*db_client {
        let query = r#"INSERT INTO member (id, first_name, last_name)
            VALUES ($1, initcap($2), initcap($3))
            ON CONFLICT (id) DO UPDATE
            SET first_name = excluded.first_name,
                last_name = excluded.last_name"#;
        let rows = c
            .query(query, &[&(*msg.author.id.as_u64() as i64), &first, &last])
            .await;
        match rows {
            Ok(r) => {
                for row in r {
                    let first_name: &str = row.get(0);
                    let last_name: &str = row.get(1);
                    println!("{} {}", first_name, last_name);
                }
            },
            Err(e) => println!("Error executing query: {:?}", e)
        }
    }


    // worry about casing in postgres

    if let Err(e) = msg.channel_id.say(&ctx.http, "OK").await {
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
