use core::panic;
use std::sync::OnceLock;

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
    pub DB_HOST: String,
    pub DB_PORT: u16,
    pub DB_NAME: String,
    pub DB_USERNAME: String,
    pub DB_PASSWORD: String,
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
        })
    }

    pub fn print_env(&self) {
        println!("DB_HOST: {}", self.DB_HOST);
        println!("DB_PORT: {}", self.DB_PORT);
        println!("DB_NAME: {}", self.DB_NAME);
        println!("DB_USERNAME: {}", self.DB_USERNAME);
        println!("DB_PASSWORD: {}", self.DB_PASSWORD);
    }
}
