use actix_web::{HttpResponse};
/****************************************************************************************
 * health_check function to respond to health check requests
 * return an HTTP 200 OK response
 */
pub async  fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
