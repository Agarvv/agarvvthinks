use actix_web::{ HttpResponse, Responder};
use serde_json::json; 
use actix_web::web; 


pub async fn health() -> impl Responder {
    let test = json!({
        "msg": "RUST"
    });
    
    HttpResponse::Ok().json(test)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
         .route("/health", web::get().to(health))
    );
}