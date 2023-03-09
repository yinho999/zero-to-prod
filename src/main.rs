use std::net::TcpListener;
use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let main_listener =
        TcpListener::bind("127.0.0.1:8000").expect("Unable to connect to main port");
    run(main_listener)?.await
}
