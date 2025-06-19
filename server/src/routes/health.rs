use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
mod healthroutes {
    pub async fn health_check() -> impl Responder {
        HttpResponse::Ok().body("RUST")
    }
}