use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

/// Returns an empty OK HttpResonse.
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Returns an HTTP server on that listens  run on the given listener
///  Currently supported routes
///   - /health_check -> returns OK and an empty body.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();

    Ok(server)
}
