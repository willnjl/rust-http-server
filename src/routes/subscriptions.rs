use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    log::info!("Saving new subscriber details in the database...");

    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id,email, name,subscribed_at)
        VALUES ($1, $2 ,$3 ,$4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            log::info!("Saving new subscriber details in the database...");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
