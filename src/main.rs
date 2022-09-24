use rust_web_hello_world::startup as s;
use rust_web_hello_world::configuration as conf;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = conf::get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}", configuration.service.host,configuration.service.port);
    let listener = TcpListener::bind(address).expect("Failed to bind service to port");
    s::run(listener)?.await
}
