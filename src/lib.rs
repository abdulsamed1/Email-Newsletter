use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use std::net::TcpListener;

/****************************************************************************************
 * spawn_app function to start the application server for testing
 * bind a TcpListener to an available port on localhost
 */
# [derive(serde::Deserialize)]
struct FormData {
    _email: String,
    _username: String,
}
async fn subscriptions(_form:web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

/****************************************************************************************
 * health_check function to respond to health check requests
 * return an HTTP 200 OK response
 */
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/****************************************************************************************
 * run recive a TcpListener and return a Result with Server or io::Error
 * create a new HttpServer with the listener
 * configure the server to respond to /health_check with the health_check function
 * return the server
 * If there is an error binding the listener, return the error

*/
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| 
        App::new().route("/health_check", web::get().to(health_check)).route("/subscriptions", web::post().to(subscriptions)))
        
        .listen(listener)?
        .run();
    Ok(server)
}
