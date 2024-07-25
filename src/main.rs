use std::net::TcpListener;

use rust_server::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 3000;
    let address = format!("127.0.0.1:{}", port);

    let listener =
        TcpListener::bind(&address).expect(&format!("Failed to bind to address:{}", &address));

    run(listener)?.await
}
