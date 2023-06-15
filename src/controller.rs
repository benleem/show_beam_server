use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Testing";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("").service(health_checker_handler);

    conf.service(scope);
}
