use actix_web::{web, HttpResponse, Responder};
use serde_json::json; // Necesitarás la crate `serde_json`

pub async fn health() -> impl Responder {
    let test = json!({
        "msg": "RUST"
    });
    
    HttpResponse::Ok().json(test)
}