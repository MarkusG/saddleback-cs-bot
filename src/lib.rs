use std::sync::Arc;
use serenity::prelude::*;
pub struct DbConnection;

impl TypeMapKey for DbConnection {
    type Value = Arc<Mutex<Option<tokio_postgres::Client>>>;
}
