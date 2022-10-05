use rust_web_hello_world::configuration as conf;
use rust_web_hello_world::startup as s;
use rust_web_hello_world::telemetry as t;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //configuration setup...
    let configuration = conf::get_configuration().expect("Failed to read configuration");

    //logging and tracing setup...
    t::init_subscriber(t::get_subscriber("rust_web_hello_world".into(), "info".into(), std::io::stdout));

    //database connection setup...
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgress");

    //service setup...
    let address = format!("{}:{}", configuration.service.host,configuration.service.port);
    let listener = TcpListener::bind(address)?;

    //kick everything off...
    s::run(listener,connection_pool)?.await
}
