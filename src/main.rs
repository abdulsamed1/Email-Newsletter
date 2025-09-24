use email_newsletter::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    match run(listener) {
        Ok(server) => server.await,
        Err(e) => {
            eprintln!("Failed to start the server: {}", e);
            Err(e)
        }
    }
}
