use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/check-health", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();

    Ok(server)
}
