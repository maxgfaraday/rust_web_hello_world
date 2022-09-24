// The collection of subscription related functionality
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("email is {:?}", form.email);
    println!("name is {:?}", form.name);
    HttpResponse::Ok().finish()
}
