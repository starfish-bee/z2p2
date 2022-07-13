use config::{Config, ConfigError, File};
use serde::Deserialize;
use tracing::{info, instrument};

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

#[instrument(level = "info")]
pub fn get_configuration() -> Result<Settings, ConfigError> {
    info!("getting config");
    let settings = Config::builder()
        .add_source(File::with_name("config"))
        .build()?;
    match settings.try_deserialize() {
        Ok(x) => {
            info!("read config successfully");
            Ok(x)
        }
        x => x,
    }
}
