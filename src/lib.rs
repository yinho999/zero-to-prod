use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, Form},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}
async fn subscriptions(req: HttpRequest, form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
