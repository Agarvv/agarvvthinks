use actix_web::Responder;
use actix_web::get;

#[get("/health")]  // <-- Añade esta macro
pub async fn health() -> impl Responder {
    "Hello world!"
}