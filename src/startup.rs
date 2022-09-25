use crate::routes as r;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

/**
Runs the HTTP Server of the API
 */
pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server,std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/",              web::get().to(r::greet))
            .route("/health_check",  web::get().to(r::health_check))
            .route("/subscriptions", web::post().to(r::subscribe))
            .app_data(connection.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}
