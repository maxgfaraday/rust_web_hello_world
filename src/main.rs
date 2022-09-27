use env_logger::Env;
use rust_web_hello_world::configuration as conf;
use rust_web_hello_world::startup as s;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = conf::get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgress");
    let address = format!("{}:{}", configuration.service.host,configuration.service.port);
    let listener = TcpListener::bind(address)?;
    s::run(listener,connection_pool)?.await
}
