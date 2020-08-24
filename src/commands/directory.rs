use log::*;

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
    if let Some(m) = ctx.data.read().await.get::<DbConnection>().cloned() {
        // wait for it to be available and lock it for our use
        let db_client = m.lock().await;
        // ensure our connection is present and run our query
        let c = &*db_client;
        let query = r#"
            INSERT INTO member (id, first_name, last_name)
            VALUES ($1, initcap($2), initcap($3))
            ON CONFLICT (id) DO UPDATE
            SET first_name = excluded.first_name,
                last_name = excluded.last_name"#;
        let rows = c
            .query(query, &[&(*msg.author.id.as_u64() as i64), &first, &last])
            .await;
        match rows {
            Ok(_) => {
                if let Err(e) = msg.channel_id.say(&ctx.http, "OK").await {
                    error!("In command name: {:?}", e);
                }
            },
            Err(e) => error!("In command name executing query: {:?}", e)
        }
    } else {
        if let Err(e) = msg.channel_id.say(&ctx.http, "Database not present").await {
            error!("In command name: {:?}", e);
        }
    }

    Ok(())
}

#[command]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let course = args.single::<String>()?.to_uppercase();
    match course.as_str() {
        "1A" | "1B" | "1C" | "1D" | "3A" | "3B" | "4A" | "CIMS140" | "CIMS150" => (),
        _ => {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Invalid course").await {
                error!("In command course: {:?}", e);
            }
            return Ok(());
        }
    }

    if let Some(m) = ctx.data.read().await.get::<DbConnection>().cloned() {
        let db_client = m.lock().await;

        let c = &*db_client;
        let query = r#"
        SELECT courses FROM member
        WHERE id = $1"#;
        let rows = c
            .query(query, &[&(*msg.author.id.as_u64() as i64)])
            .await;
        let mut courses = Vec::new();
        match rows {
            Ok(r) => {
                for row in r {
                    if let Ok(c) = row.try_get(0) {
                        courses = c;
                        if courses.contains(&course) {
                            if let Err(e) = msg.channel_id.say(&ctx.http, "Already in course").await {
                                error!("In command course add: {:?}", e);
                            }
                            return Ok(());
                        }
                    }
                }

                if !courses.contains(&course) {
                    courses.push(course.to_string());
                } else {
                    if let Err(e) = msg.channel_id.say(&ctx.http, "Already in course").await {
                        error!("In command course add: {:?}", e);
                    }
                }
            },
            Err(e) => {
                error!("In command course add executing query: {:?}", e);
                return Ok(())
            }
        }
        let query = r#"
        INSERT INTO member (id, courses)
        VALUES ($1, $2)
        ON CONFLICT (id) DO UPDATE
        SET courses = excluded.courses"#;
        let rows = c
            .query(query, &[&(*msg.author.id.as_u64() as i64), &courses])
            .await;
        match rows {
            Ok(_) => {
                if let Err(e) = msg.channel_id.say(&ctx.http, "OK").await {
                    error!("In command course add: {:?}", e);
                }
            },
            Err(e) => error!("In command course add executing query: {:?}", e)
        }
    } else {
        if let Err(e) = msg.channel_id.say(&ctx.http, "Database not present").await {
            error!("In command course add: {:?}", e);
        }
    }
    Ok(())
}

#[command]
pub async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let course = args.single::<String>()?.to_uppercase();
    match course.as_str() {
        "1A" | "1B" | "1C" | "1D" | "3A" | "3B" | "4A" | "CIMS140" | "CIMS150" => (),
        _ => {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Invalid course").await {
                error!("In command course: {:?}", e);
            }
            return Ok(());
        }
    }

    if let Some(m) = ctx.data.read().await.get::<DbConnection>().cloned() {
        let db_client = m.lock().await;

        let c = &*db_client;
        let query = r#"
        SELECT courses FROM member
        WHERE id = $1"#;
        let rows = c
            .query(query, &[&(*msg.author.id.as_u64() as i64)])
            .await;
        let mut courses = Vec::new();
        match rows {
            Ok(r) => {
                for row in r {
                    if let Ok(c) = row.try_get(0) {
                        courses = c;
                        if !courses.contains(&course) {
                            if let Err(e) = msg.channel_id.say(&ctx.http, "Not in course").await {
                                error!("In command course remove: {:?}", e);
                            }
                            return Ok(());
                        }
                    }
                }

                if courses.len() == 0 {
                    if let Err(e) = msg.channel_id.say(&ctx.http, "Not in course").await {
                        error!("In command course remove: {:?}", e);
                    }
                    return Ok(());

                }
                courses.retain(|c| c != &course.to_string());
                let query = r#"
                INSERT INTO member (id, courses)
                VALUES ($1, $2)
                ON CONFLICT (id) DO UPDATE
                SET courses = excluded.courses"#;
                let rows = c
                    .query(query, &[&(*msg.author.id.as_u64() as i64), &courses])
                    .await;
                match rows {
                    Ok(_) => {
                        if let Err(e) = msg.channel_id.say(&ctx.http, "OK").await {
                            error!("In command course remove: {:?}", e);
                        }
                    },
                    Err(e) => error!("In command course remove executing query: {:?}", e)
                }
            },
            Err(e) => {
                error!("In command course remove executing query: {:?}", e);
                return Ok(())
            }
        }
    } else {
        if let Err(e) = msg.channel_id.say(&ctx.http, "Database not present").await {
            error!("In command course remove: {:?}", e);
        }
    }
    Ok(())
}
