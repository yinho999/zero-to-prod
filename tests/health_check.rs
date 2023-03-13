use std::net::TcpListener;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use zero2prod::{
    configuration::{self, DatabaseSettings},
    startup::run,
};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Unable to bind random port");
    // Get the port from the listener
    let port = listener.local_addr().unwrap().port();
    println!("{}", listener.local_addr().unwrap());

    // Database Pool setup
    let mut config = configuration::get_configuration().expect("Failed to read configuration file");

    // Setup random testing database name
    config.database.database_name = uuid::Uuid::new_v4().to_string();

    // Create and migrate test database using random database name
    let db_pool = config_test_database(&config.database).await;

    // Setup server with Postgres Pool
    let server = run(listener, db_pool.clone()).expect("Unable to bind address to server");

    // Run Test server in the background
    let _ = actix::spawn(server);

    // Return TestApp contains application state
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool,
    }
}

async fn config_test_database(database_config: &DatabaseSettings) -> PgPool {
    // Get Postgres Instance
    let instance_connection_str = database_config.connection_string_without_database();
    let mut pg_instance = PgConnection::connect(&instance_connection_str)
        .await
        .expect("Failed to connect to Postgres");

    // Create test database with random name via Postgres Instance
    pg_instance
        .execute(format!(r#"CREATE DATABASE "{}";"#, database_config.database_name).as_str())
        .await
        .expect("Failed to create database");

    // Connect to test database and Migrate test database
    let test_db_pool = PgPool::connect(&database_config.connection_string_with_database())
        .await
        .expect("Failed to connect Postgres Database");

    // Run Migration
    sqlx::migrate!("./migrations")
        .run(&test_db_pool)
        .await
        .expect("Failed to run migrations test database");
    test_db_pool
}
#[actix_web::test]
async fn health_check_works() {
    // Spawn Server
    let app = spawn_app().await;
    // Create new client
    let client = reqwest::Client::new();

    // Client Act on Server
    // the health check is exposed at /health_check;
    // the health check is behind a GET method;
    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Compare response
    // the health check always returns a 200;
    // the health check’s response has no body;
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_web::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange server
    let app = spawn_app().await;

    // Spawn client
    let client = reqwest::Client::new();

    // Make Request
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send the request");
    // Compare Expected Response and Actual Response
    assert_eq!(200, response.status().as_u16());
    // Get saved query
    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}
#[actix_web::test]
async fn subscribe_return_a_400_when_data_is_missing() {
    // Arrange server
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send the request");
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
