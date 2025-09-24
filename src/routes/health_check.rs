use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};
use std::net::TcpListener;
/****************************************************************************************
 * health_check function to respond to health check requests
 * return an HTTP 200 OK response
 */
pub async  fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
