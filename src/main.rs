use rust_web_hello_world;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    rust_web_hello_world::run()?.await
}
