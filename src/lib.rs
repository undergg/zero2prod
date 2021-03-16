use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::{TcpListener};

async fn health_check(_ : HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

// We need to mark "run" as public so that we can share it with main.rs
pub fn run(listener : TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}