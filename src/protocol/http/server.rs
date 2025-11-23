use std::net::SocketAddr;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use loggix::with_fields;
use super::handler::{self, DataJson};

pub async fn run(addr: String) -> std::io::Result<()> {

    with_fields!(
       "addr".to_string() => &addr
    ).info("http server is active");

    let addr: SocketAddr = addr.parse().unwrap();


    HttpServer::new(|| {
        App::new()
            .route("/get", web::get().to(handler::get)) // Define a route for GET requests to "/"
            .route("/set", web::post().to(handler::set))
    })
    .bind(addr)? // Bind the server to an address and port
    .run() // Start the server
    .await // Await the server's completion (e.g., on Ctrl-C)
}
