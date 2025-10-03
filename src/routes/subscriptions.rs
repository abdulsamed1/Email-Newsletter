use actix_web::{web, HttpResponse};
use sqlx::PgConnection;
/****************************************************************************************
 * spawn_app function to start the application server for testing
 * bind a TcpListener to an available port on localhost
 */
#[derive(serde::Deserialize)]
pub struct FormData {
    _email: String,
    _username: String,
}
pub async fn subscriptions(
    _form: web::Form<FormData>,
    _connection: web::Data<PgConnection>,
) -> HttpResponse {
    
    HttpResponse::Ok().finish()
}
