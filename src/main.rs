use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // Get configuration from configuration file
    let config = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", config.application_port);
    let main_listener = TcpListener::bind(address)?;
    run(main_listener)?.await
}
