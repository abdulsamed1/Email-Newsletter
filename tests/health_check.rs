//tests/health_check.rs

use email_newsletter::configuration::get_configuration;
use sqlx::{Connection, PgConnection};



fn spawn_app() -> String {
    /**
 * Spawn the application and return the address to it
 *  it make same as main but in test env
 * create a TcpListener bind it to random port
 * get the port from the listener
 * run the server with the listener
 * spawn the server as a new task
 * return the address to the server
 */
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap();

    let server = email_newsletter::startup::run(listener).expect("Failed to run server");
    let _ = tokio::spawn(server);
    format!("htttp://127.0.0.1:{}", port)
    //port.to_string()
}

//********************************************* */
#[tokio::test]

async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

/****************************************************************************************/

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange

    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());
}
/****************************************************************************************/

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
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
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
    }
}
