use std::net::TcpListener;

fn spawn_app() -> String {
    let ip = "127.0.0.1";
    let listener = TcpListener::bind(format!("{}:0",ip)).expect("Failed To Bind random port");
    let port = listener.local_addr().unwrap().port();

    println!("Spawning server on port: {}", port);

    let server = rust_web_hello_world::run(listener).expect("Failed to bind to address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}",port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
