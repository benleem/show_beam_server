use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use sqlx::mysql;

#[get("")]
async fn get_all_users() -> impl Responder {
    const MESSAGE: &str = "Hello from get all users";
    const data: mysql::MySqlPool = HttpRequest::app_data::<mysql::MySqlPool>;

    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}

#[post("")]
async fn new_user() -> impl Responder {
    // const MESSAGE: &str = "Hello, world!";

    HttpResponse::Ok()
}

#[put("")]
async fn edit_user() -> impl Responder {
    // const MESSAGE: &str = "Hello, world!";

    HttpResponse::Ok()
}

#[delete("")]
async fn delete_user() -> impl Responder {
    // const MESSAGE: &str = "Hello, world!";

    HttpResponse::Ok()
}

// config

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .service(get_all_users)
        .service(new_user)
        .service(edit_user)
        .service(delete_user);

    conf.service(scope);
}
