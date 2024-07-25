use actix_web::{
    dev::Server, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
