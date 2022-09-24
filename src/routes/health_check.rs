// The collection of heath check related functionality
use actix_web::{HttpRequest, Responder, HttpResponse};

/**
This is our initial API call to greet the user
 */
pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}, Welcome to this service!", &name)
}

/**
Performs the health check for this system
 */
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
