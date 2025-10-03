use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

use crate::routes::{health_check, subscriptions};

/****************************************************************************************
 * run recive a TcpListener and return a Result with Server or io::Error
 * create a new HttpServer with the listener
 * configure the server to respond to /health_check with the health_check function
 * return the server
 * If there is an error binding the listener, return the error

*/
pub fn run(connection: PgConnection, listener: TcpListener) -> Result<Server, std::io::Error> {
   let connection=web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions)).
            app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
