use crate::routes::*;

use actix_web::{dev::Server, get, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
