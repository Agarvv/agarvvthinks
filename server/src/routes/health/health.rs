
use actix_web::web; 
use crate::handlers::health::health;

pub fn configure(cfg: &mut web::ServiceConfig) {
     cfg.route("/health", web::get().to(health));
}