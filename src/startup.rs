use crate::routes::{health_check, subscriptions};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{PgConnection, PgPool};
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    database_connection_pool: PgPool,
) -> Result<Server, std::io::Error> {
    // Create shared state data for application
    let database_connection_pool = web::Data::new(database_connection_pool);

    // Create Server
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
            .app_data(database_connection_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
