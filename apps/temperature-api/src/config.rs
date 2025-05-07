use config::{Config, ConfigError, Environment, File, FileFormat};
use dotenv::dotenv;
use getset::Getters;
use serde::Deserialize;

use crate::logger::LoggerConfig;
use crate::server::config::ServerConfig;

const DEV_FILE_CONFIG_PATH: &str = "./config/development.toml";
const SERVICE_RUN_MODE: &str = "TEMPERATURE_API__RUN_MODE";
const SERVICE_PREFIX: &str = "TEMPERATURE_API";

#[derive(Deserialize, Getters)]
#[getset(get = "pub")]
pub struct ServiceConfig {
    logger: LoggerConfig,
    server: ServerConfig,
}

impl ServiceConfig {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();

        let dev_file_config = File::with_name(DEV_FILE_CONFIG_PATH);

        let run_mode = std::env::var(SERVICE_RUN_MODE).unwrap_or("development".into());
        let run_mode_file_path = format!("./config/{}", run_mode);
        let file_config = File::with_name(&run_mode_file_path)
            .format(FileFormat::Toml)
            .required(false);

        let env_config = Environment::with_prefix(SERVICE_PREFIX)
            .separator("__")
            .try_parsing(true);

        let settings = Config::builder()
            .add_source(dev_file_config)
            .add_source(file_config)
            .add_source(env_config)
            .build()?;

        settings.try_deserialize()
    }
}
