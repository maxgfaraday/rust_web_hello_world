use crate::routes as r;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

/**
Runs the HTTP Server of the API
 */
pub fn run(listener: TcpListener) -> Result<Server,std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/",              web::get().to(r::greet))
            .route("/health_check",  web::get().to(r::health_check))
            .route("/subscriptions", web::post().to(r::subscribe))
    })
        .listen(listener)?
        .run();
    Ok(server)
}
