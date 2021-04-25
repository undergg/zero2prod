use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, HttpServer};
use actix_web::{web, App};
use sqlx::PgPool;
use std::net::TcpListener;

// We need to mark "run" as public so that we can share it with main.rs
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    // HttpServer::new does not take App as argument - it wants a closure that returns an App struct.
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
