use actix_web::{ post, web, HttpResponse, Responder };
use chrono::Utc;
use sqlx::{ query, PgPool };
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    let request_id = Uuid::new_v4();

    let request_span =
        tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name= %form.name
    );

    let request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );

    tracing::info!("request_id {} - Saving new subscriber details in the database", request_id);

    let result = sqlx::query!(
            r#"
                INSERT INTO subscriptions (id,email, name,subscribed_at)
                VALUES ($1, $2 ,$3 ,$4)
            "#,
            Uuid::new_v4(),
            form.email,
            form.name,
            Utc::now()
        )
        .execute(db_pool.get_ref())
        .instrument(query_span).await;

    match result {
        Ok(_) => {
            tracing::info!("request_id {} - New subscriber details have been saved", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
