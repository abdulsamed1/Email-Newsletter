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

/****************************************************************************************
 * Spawn the application and return the address to it
 *  it make same as main but in test env
 * create a TcpListener bind it to random port
 * get the port from the listener
 * run the server with the listener
 * spawn the server as a new task
 * return the address to the server
 */
fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
    let port = listener.local_addr().unwrap();

    let server = email_newsletter::run(listener).expect("Failed to run server");
    let _ = tokio::spawn(server);
    format!("htttp://127.0.0.1:{}", port)
    //port.to_string()
}
