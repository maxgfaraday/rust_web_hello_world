use rust_web_hello_world::configuration as conf;
use rust_web_hello_world::startup as s;

use tracing::{Subscriber,subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subcriber: impl Subscriber + Send + Sync){
    LogTracer::init().expect("Failed to get Logger");
    set_global_default(subcriber).expect("Failed to set subscriber");
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //configuration setup...
    let configuration = conf::get_configuration().expect("Failed to read configuration");

    //logging and tracing setup...
    init_subscriber(get_subscriber("rust_web_hello_world".into(), "info".into()));

    //database connection setup...
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgress");

    //service setup...
    let address = format!("{}:{}", configuration.service.host,configuration.service.port);
    let listener = TcpListener::bind(address)?;

    //kick everything off...
    s::run(listener,connection_pool)?.await
}
