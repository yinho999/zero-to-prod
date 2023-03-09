use std::net::TcpListener;

use zero2prod::run;
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Unable to bind random port");
    // Get the port from the listener
    let port = listener.local_addr().unwrap().port();
    println!("{}", listener.local_addr().unwrap());
    let server = run(listener).expect("Unable to bind address to server");
    let _ = actix::spawn(server);

    // Return address with port
    format!("http://127.0.0.1:{}", port)
}

#[actix_web::test]
async fn health_check_works() {
    // Spawn Server
    let address = spawn_app();
    // Create new client
    let client = reqwest::Client::new();

    // Client Act on Server
    // the health check is exposed at /health_check;
    // the health check is behind a GET method;
    let response = client
        .get(format!("{address}/health_check"))
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
    let app_address = spawn_app();
    // Spawn client
    let client = reqwest::Client::new();

    // Make Request
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{app_address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send the request");
    assert_eq!(200, response.status().as_u16());
}
#[actix_web::test]
async fn subscribe_return_a_400_when_data_is_missing() {
    // Arrange server
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{app_address}/subscriptions"))
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
