use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("")]
async fn get_all_shows() -> impl Responder {
    const MESSAGE: &str = "Hello from get all shows";

    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}

#[post("")]
async fn new_show() -> impl Responder {
    const MESSAGE: &str = "Hello from new show";

    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}

#[put("")]
async fn edit_show() -> impl Responder {
    const MESSAGE: &str = "Hello from edit show";

    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}

#[delete("")]
async fn delete_show() -> impl Responder {
    const MESSAGE: &str = "Hello from delete show";

    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}

// /shows/users

#[get("/users")]
async fn get_all_user_shows() -> impl Responder {
    const MESSAGE: &str = "Hello from get user shows";

    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}

// config

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/shows")
        .service(get_all_shows)
        .service(new_show)
        .service(edit_show)
        .service(delete_show)
        .service(get_all_user_shows);

    conf.service(scope);
}
