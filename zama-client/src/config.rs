use std::{env, sync::OnceLock};
use crate::Result;

#[allow(non_snake_case)]
pub struct Config {
    pub server_url: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            server_url: Self::get_env("SERVER_URL")?,
        })
    }

    pub fn config() -> &'static Config {
        dotenv::dotenv().ok();

        static INSTANCE: OnceLock<Config> = OnceLock::new();
    
        INSTANCE.get_or_init(|| {
            Config::load_from_env().unwrap_or_else(|ex| {
                panic!("FATAL - HWILE LOADING CONF - Cause: {ex:?}")
            })
        })
    }

    fn get_env(name: &'static str) -> Result<String, std::env::VarError> {
        env::var(name)
    }
}

