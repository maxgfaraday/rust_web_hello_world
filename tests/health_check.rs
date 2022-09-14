use std::net::TcpListener;

fn spawn_app() -> String {
    let ip = "127.0.0.1";
    let port = "8000";
    let url = format!("{}:{}",ip,port);
    let server = rust_web_hello_world::run(&url).expect("Failed to bind to address");
    let _ = tokio::spawn(server);
    url
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
