use rust_web_hello_world::startup;
use rust_web_hello_world::configuration::get_configuration;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to port");
    startup::run(listener)?.await
}
