use crate::routes::health_check;
use crate::routes::subscribe;
use actix_web::{dev::Server, HttpServer};
use actix_web::{web, App};
use std::net::TcpListener;

// We need to mark "run" as public so that we can share it with main.rs
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
