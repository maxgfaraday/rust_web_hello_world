use rust_web_hello_world::startup as s;
use rust_web_hello_world::configuration as conf;
use sqlx::{Connection,PgConnection};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = conf::get_configuration().expect("Failed to read configuration");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgress");
    let address = format!("{}:{}", configuration.service.host,configuration.service.port);
    let listener = TcpListener::bind(address)?;
    s::run(listener,connection)?.await
}
