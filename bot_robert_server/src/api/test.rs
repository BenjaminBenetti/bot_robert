use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}