use std::sync::Arc;
use serenity::prelude::*;
pub struct DbConnection;

impl TypeMapKey for DbConnection {
    type Value = Arc<Mutex<tokio_postgres::Client>>;
}
