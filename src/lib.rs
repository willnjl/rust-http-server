use actix_web::{
    dev::Server, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(subscribe)
            .route("/health-check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
