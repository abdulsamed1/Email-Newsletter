use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{ PgPool};
use std::net::TcpListener;

use crate::routes::{health_check, subscriptions};

/****************************************************************************************
 * run recive a TcpListener and return a Result with Server or io::Error
 * create a new HttpServer with the listener
 * configure the server to respond to /health_check with the health_check function
 * return the server
 * If there is an error binding the listener, return the error

*/
pub fn run(db_pool: PgPool, listener: TcpListener) -> Result<Server, std::io::Error> {
   let db_pool=web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions)).
            app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
