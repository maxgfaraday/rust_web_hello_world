use std::net::TcpListener;
use rust_web_hello_world::startup;
use rust_web_hello_world::configuration::get_configuration;
use sqlx::PgPool;

pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool
}

async fn spawn_app() -> TestApp {
    let configuration = get_configuration().expect("Failed to load configuration file");
    let host = configuration.service.host;
    let listener = TcpListener::bind(format!("{}:0",host)).expect("Failed To Bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://{}:{}", host, port);
    println!("Spawning server on port: {}", port);

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let server = startup::run(listener,connection_pool.clone()).expect("Failed to bind to address");
    let _ = tokio::spawn(server);
    println!("{}",address);

    TestApp {
        address,
        connection_pool
    }
}

#[tokio::test]
async fn health_check_works() {
    let app_info = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app_info.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //fire up the service
    let app_info = spawn_app().await;
    let app_address = app_info.address;
    let client = reqwest::Client::new();

    //send our service data.
    let body ="name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute subscribe request.");

    assert_eq!(200,response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app_info.connection_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app_info = spawn_app().await;
    let app_address = app_info.address;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com","missing the name"),
        ("", "missing both name and email")];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute subscribe request.");

        assert_eq!(400,
                   response.status().as_u16(),
                   "The API did not fail with 400 Bad Request, as it shoudld, when the payload was: {}", error_message);
    }
}
