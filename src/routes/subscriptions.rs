use actix_web::web::Form;
use actix_web::{HttpRequest, HttpResponse};

use serde::Deserialize;
#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn subscriptions(req: HttpRequest, form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
