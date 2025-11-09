use std::net::TcpListener;
use actix_web::{App, HttpResponse};
use actix_web::dev::Server;
use actix_web::web;
use actix_web::HttpServer;


async fn health_check() ->  HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new()
        .route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();
    Ok(server)
}