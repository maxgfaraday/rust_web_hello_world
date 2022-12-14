use serde_aux::field_attributes::deserialize_number_from_string;
use secrecy::{Secret, ExposeSecret};
use sqlx::{postgres::{PgConnectOptions, PgSslMode}, ConnectOptions};

#[derive (serde::Deserialize)]
pub struct Settings{
    pub service: ServiceSettings,
    pub database: DatabaseSettings
}

#[derive (serde::Deserialize)]
pub struct ServiceSettings{
    pub host: String,
    #[serde (deserialize_with = "deserialize_number_from_string")]
    pub port: u16
}

#[derive (serde::Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    #[serde (deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub database_name: String,
    pub require_ssl: bool
}

impl DatabaseSettings {
    //gets you the connection string...
    pub fn connection_string_wo_database(&self) -> Secret<String> {
        Secret::new(format!("postgres://{}:{}@{}:{}",
                            self.username,
                            self.password.expose_secret(),
                            self.host,
                            self.port))
    }

    //(preferred)
    //gets you an object that represents the same information as the connection string
    pub fn without_db(&self) -> PgConnectOptions{
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        }else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .username(&self.username)
            .password(&self.password.expose_secret())
            .host(&self.host)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    //gets you the connection string... additionally with the database_name included
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!("postgres://{}:{}@{}:{}/{}",
                            self.username,
                            self.password.expose_secret(),
                            self.host,
                            self.port,
                            self.database_name))
    }

    //(preferred)
    //gets you an object that represents the same information as the connection string
    //additionally with the database_name incuded
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options.log_statements(tracing::log::LevelFilter::Trace);
        options
    }
}


pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let config_directory = base_path.join("configuration");
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT. WTF, Bitch!");
    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(config_directory.join("base.yaml")))
        .add_source(config::File::from(config_directory.join(&environment_filename)))
        .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"))
        .build()?;
    settings.try_deserialize::<Settings>()
}

//--------------------------
// Environment definition object
//--------------------------

pub enum Environment {
    Local,
    Production
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production"
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!("{} is not a supported environment. Use either `local` or `production`.", other))
        }
    }
}
