use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // Get configuration from configuration file
    let config = get_configuration().expect("Failed to read configuration");
    let database_connection_pool =
        PgPool::connect(&config.database.connection_string_with_database())
            .await
            .expect("Failed to connect to Postgres Database");
    let address = format!("127.0.0.1:{}", config.application_port);
    let main_listener = TcpListener::bind(address)?;
    run(main_listener, database_connection_pool)?.await
}
