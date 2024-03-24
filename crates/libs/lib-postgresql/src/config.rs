use core::panic;
use std::{sync::OnceLock, time::Duration};

// region:    --- Modules
use lib_utils::envs::{get_env, get_env_parse};
// endregion: --- Modules

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONFIG - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    // -- Db
    DB_HOST: String,
    DB_PORT: u16,
    DB_NAME: String,
    DB_USERNAME: String,
    DB_PASSWORD: String,
    DB_CONNECTION_TIMEOUT: u64,
    DB_CONNECTION_POOLS: u32,
}

impl CoreConfig {
    fn load_from_env() -> lib_utils::envs::Result<CoreConfig> {
        Ok(CoreConfig {
            // -- Db
            DB_HOST: get_env("POSTGRES_HOST")?,
            DB_PORT: match get_env_parse::<u16>("POSTGRES_PORT") {
                Ok(port) => port,
                Err(_) => 1433,
            },
            DB_NAME: get_env("POSTGRES_DB")?,
            DB_USERNAME: get_env("POSTGRES_USER")?,
            DB_PASSWORD: get_env("POSTGRES_PASSWORD")?,
            DB_CONNECTION_TIMEOUT: match get_env_parse::<u64>("POSTGRES_IDLE_TIMEOUT") {
                Ok(idle_timeout) => idle_timeout,
                Err(_) => 3600,
            },
            DB_CONNECTION_POOLS: match get_env_parse::<u32>("POSTGRES_POOLS") {
                Ok(pools) => pools,
                Err(_) => 3,
            },
        })
    }

    pub fn get_connection_str(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.DB_USERNAME, self.DB_PASSWORD, self.DB_HOST, self.DB_NAME
        )
    }

    pub fn get_connection_timeout(&self) -> Duration {
        Duration::from_secs(self.DB_CONNECTION_TIMEOUT)
    }

    pub fn get_connection_pools(&self) -> u32 {
        self.DB_CONNECTION_POOLS
    }

    pub fn print_env(&self) {
        println!("DB_HOST: {}", self.DB_HOST);
        println!("DB_PORT: {}", self.DB_PORT);
        println!("DB_NAME: {}", self.DB_NAME);
        println!("DB_USERNAME: {}", self.DB_USERNAME);
        println!("DB_PASSWORD: {}", self.DB_PASSWORD);
    }
}
