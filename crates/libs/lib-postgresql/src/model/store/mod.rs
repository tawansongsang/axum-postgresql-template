// region:    --- Modules
mod error;

pub use self::error::{Error, Result};

use crate::config::core_config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

// endregion: --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    let connection_str = core_config().get_connection_str();
    let idle_timeout = core_config().get_connection_timeout();
    let max_connections = core_config().get_connection_pools();
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .idle_timeout(idle_timeout)
        .connect(connection_str.as_str())
        .await?;

    Ok(pool)
}
