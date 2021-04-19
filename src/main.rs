use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bubble op the io::Error if we failed to bind the address.
    // Otherwise call .await on our Server.
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
