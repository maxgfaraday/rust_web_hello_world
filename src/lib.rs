use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

/**
This is our initial API call to greet the user
*/
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}, Welcome to this service!", &name)
}

/**
Performs the health check for this system
*/
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/**
Runs the HTTP Server of the API
*/
pub fn run(listener: TcpListener) -> Result<Server,std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/",              web::get().to(greet))
            .route("/health_check",  web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();
    Ok(server)
}
