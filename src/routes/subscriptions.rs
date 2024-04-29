/// /subscriptions handlers.
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

/// Subscribes a user to the newsletter
///  - Preconditions
///     * email and name set.
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
       INSERT INTO subscriptions (id, email, name, subscribed_at)
       VALUES($1, $2, $3, $4)
       "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("failed to exec query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
