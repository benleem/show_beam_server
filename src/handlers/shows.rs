use crate::{
    models::shows::{
        CreateShowSchema, DeleteShowSchema, GetUserShowsSchema, ShowModel, UpdateShowSchema,
    },
    AppState,
};

use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json, Path, Query},
    HttpResponse, Responder,
};
use serde_json::json;

// endpoints

#[get("")]
async fn get_all_shows(data: Data<AppState>) -> impl Responder {
    let shows: Vec<ShowModel> = sqlx::query_as!(ShowModel, "SELECT * FROM shows")
        .fetch_all(&data.db)
        .await
        .unwrap();
    let json_response = serde_json::json!({ "shows": shows });
    HttpResponse::Ok().json(json!(json_response))
}

#[get("/{id}")]
async fn get_show_by_id(path: Path<(String,)>, data: Data<AppState>) -> HttpResponse {
    let show: ShowModel = sqlx::query_as!(
        ShowModel,
        "SELECT * FROM shows WHERE id = ?",
        path.into_inner().0
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    let json_response = serde_json::json!({ "show": show });
    HttpResponse::Ok().json(json!(json_response))
}

#[get("/users/{id}")]
async fn get_all_user_shows(
    path: Path<(String,)>,
    params: Query<GetUserShowsSchema>,
    data: Data<AppState>,
) -> impl Responder {
    // if params.favorites {
    //     //do something
    // }
    let shows: Vec<ShowModel> = sqlx::query_as!(
        ShowModel,
        "SELECT * FROM shows WHERE owner = ?",
        path.into_inner().0
    )
    .fetch_all(&data.db)
    .await
    .unwrap();
    let json_response = serde_json::json!({ "shows": shows });
    HttpResponse::Ok().json(json!(json_response))
}

#[post("")]
async fn new_show(body: Json<CreateShowSchema>, data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(json!(body))
}

#[put("")]
async fn edit_show(body: Json<UpdateShowSchema>, data: Data<AppState>) -> impl Responder {
    const MESSAGE: &str = "Hello, world!";

    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}

#[delete("")]
async fn delete_show(params: Query<DeleteShowSchema>, data: Data<AppState>) -> impl Responder {
    const MESSAGE: &str = "Hello, world!";

    HttpResponse::Ok().json(json!({ "message": MESSAGE }))
}

// config

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/shows")
        .service(get_all_shows)
        .service(get_show_by_id)
        .service(new_show)
        .service(edit_show)
        .service(delete_show)
        .service(get_all_user_shows);
    conf.service(scope);
}
