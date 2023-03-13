use actix_web::web::{self, Form};
use actix_web::{HttpRequest, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(
    _req: HttpRequest,
    db_pool: web::Data<PgPool>,
    form: Form<FormData>,
) -> HttpResponse {
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.as_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query, {}", e.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}
