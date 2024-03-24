mod error;
mod store;
pub mod task;
pub mod user_info;

use tracing::info;

use self::error::Result;
use self::store::{new_db_pool, Db};

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Contructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    // pub(in crate::model) fn db(&self) -> &Db {
    pub fn db(&self) -> &Db {
        &self.db
    }

    pub async fn test_connection(&self) {
        info!(
            "{:<12} - test_connection - {}",
            "STARTUP", "trying to use pool for connecting to sql server"
        );

        let pool = self.db();

        let row: (String,) = sqlx::query_as("SELECT VERSION();")
            .fetch_one(pool)
            .await
            .unwrap();

        info!(
            "{:<12} - test_connection {}: \n{:?}",
            "STARTUP", "Postgres Server Version: ", row.0
        );
    }
}
