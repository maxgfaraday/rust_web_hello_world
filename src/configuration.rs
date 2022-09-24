#[derive(serde::Deserialize)]
pub struct Settings{
    pub service: ServiceSettings,
    pub database: DatabaseSettings
}

#[derive(serde::Deserialize)]
pub struct ServiceSettings{
    pub host: String,
    pub port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Settings>()
}
