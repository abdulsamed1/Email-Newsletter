use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use std::net::TcpListener;

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
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();
    Ok(server)
}
