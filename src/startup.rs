use crate::routes as r;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

/**
Runs the HTTP Server of the API
 */
pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server,std::io::Error> {
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/",              web::get().to(r::greet))
            .route("/health_check",  web::get().to(r::health_check))
            .route("/subscriptions", web::post().to(r::subscribe))
            .app_data(connection_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}
