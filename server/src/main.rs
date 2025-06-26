use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use std::env;
use sea_orm::{Database};


pub mod routes;
pub mod handlers;
pub mod services;
pub mod entities; 


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("kein DATABASE_URL");
    let db = Database::connect(&database_url).await.expect("Fehler");
    
    

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .configure(routes::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
