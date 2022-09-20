use rust_web_hello_world;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port 8000");
    rust_web_hello_world::run(listener)?.await
}
