use actix_web::web; 

#[path = "health/health.rs"]
pub mod health;



pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/agarvvthinks")
         .configure(health::configure)
    );
}