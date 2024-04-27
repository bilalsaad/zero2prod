/// /subscriptions handlers.
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

/// Subscribes a user to the newsletter
///  - Preconditions
///     * email and name set.
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
