use actix_web::{guard::Connect, web, HttpResponse};
use sqlx::{Connection, PgConnection};
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
    sqlx::query!(
        r#"
insert into subscriptions (id, email, username, subscribed_at)
values ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form._email,
        form._username,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await;
    HttpResponse::Ok().finish()
}
