use once_cell::sync::Lazy;
use rust_web_hello_world::configuration as conf;
use rust_web_hello_world::startup as s;
use rust_web_hello_world::telemetry as t;
use secrecy::ExposeSecret;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use std::net::TcpListener;
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        t::init_subscriber(t::get_subscriber("test".into(), "debug".into(), std::io::stdout));
    }else{
        t::init_subscriber(t::get_subscriber("test".into(), "debug".into(), std::io::sink));
    }
});

pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool
}

async fn spawn_app() -> TestApp {
    let mut configuration = conf::get_configuration().expect("Failed to load configuration file");
    let host = configuration.service.host;

    //---------------------
    // Telemetry setup (wrapped in a singleton <above>)
    //---------------------

    Lazy::force(&TRACING);


    //---------------------
    // Service setup
    //---------------------

    let listener = TcpListener::bind(format!("{}:0",host)).expect("Failed To Bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://{}:{}", host, port);
    println!("Spawning server on port: {}", port);

    //---------------------
    // Database setup
    //---------------------

    //modify the configuration to have a new,unique database_name
    configuration.database.database_name = format!("_test_{}",Uuid::new_v4().to_string());
    let connection_pool = configure_database(&configuration.database).await;

    println!("Testing Database: {:#?}",configuration.database.connection_string());

    let server = s::run(listener,connection_pool.clone()).expect("Failed to bind to address");
    let _ = tokio::spawn(server);
    println!("{}",address);

    TestApp {
        address,
        connection_pool
    }
}

pub async fn configure_database(config: &conf::DatabaseSettings) -> PgPool {
    //Connect to postgres....
    let mut connection = PgConnection::connect(&config.connection_string_wo_database().expose_secret()).await.expect("Failed to connect to Postgres");

    //Create database
    connection.execute(format!(r#"CREATE DATABASE "{}";"#,config.database_name).as_str()).await.expect("Failed to create database");

    //Migrate database
    let connection_pool = PgPool::connect(&config.connection_string().expose_secret()).await.expect("Failed to connect to Postgres");
    sqlx::migrate!("./migrations").run(&connection_pool).await.expect("Failed to migrate the database");

    connection_pool
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
