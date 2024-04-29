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
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, db_pool),
    fields(request_id = %Uuid::new_v4(),
    subscriber_email = %form.email,
    subscriber_name = %form.name
    )
)]

pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&form, &db_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new subscriber to database.", skip(form, pool))]
async fn insert_subscriber(form: &FormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
       INSERT INTO subscriptions (id, email, name, subscribed_at)
       VALUES($1, $2, $3, $4)
       "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to exec query: {:?}", e);
        e
    })?;
    Ok(())
}
