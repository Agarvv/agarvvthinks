use actix_web::{HttpResponse, Responder};
use serde_json::{json, Value};
use crate::services::health_service::h_check;

// Jaaaa, ich weiß dieser Code ist Scheiße
// Ich habe Lust mit Rust zu spielen und zu lernen... ach, es ist mir scheißegal.
// ES MACHT EINFACH SPAß, OKAY???!!!
// ICH MUSS NIEMANDEM ERKLÄRUNGEN GEBEN
// Außerdem rede ich nur mit mir selbst, ich bin ein dummkopf 


pub async fn health() -> impl Responder {
    let mut test = json!({
        "msg": "RUST"
    });

    let num = h_check();

    match num {
        5 => HttpResponse::Ok().json(test),
        _ => {
            test["msg"] = Value::String("Scheiße.".to_string());
            HttpResponse::BadRequest().json(test)
        }
    }
}
