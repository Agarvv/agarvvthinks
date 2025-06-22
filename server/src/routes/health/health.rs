use actix_web::{ HttpResponse, Responder};
use serde_json::json; 
use actix_web::web; 
use crate::handlers::health::health;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
         .route("/health", web::get().to(health))
    );
}