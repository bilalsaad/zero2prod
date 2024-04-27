/// /health_check route handler
///
use actix_web::HttpResponse;

/// Returns an empty OK HttpResonse.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
