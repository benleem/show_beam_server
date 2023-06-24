use crate::models::auth::AppState;
use crate::models::users::UserModel;
use actix_web::dev::Path;
use actix_web::web::Data;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use sqlx;

#[get("")]
async fn get_all_users(_data: Data<AppState>) -> impl Responder {
    const MESSAGE: &str = "Hello from get all users";

    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}

#[get("/{id}")]
async fn get_user_by_id(path: web::Path<(String,)>, data: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = ?")
        .bind(&path.0)
        .fetch_optional(&data.db)
        .await
    {
        Ok(Some(user)) => {
            let json_response = json!({ "user": user });
            HttpResponse::Ok().json(json_response)
        }
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(error) => {
            eprintln!("Failed to retrieve user: {}", error);
            HttpResponse::InternalServerError().body("Failed to retrieve user")
        }
    }
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
