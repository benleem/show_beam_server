use crate::models::{
    app::AppState,
    slides::{CreateSlideBody, DeleteSlideParams, SlideModelSql, UpdateSlideBody},
};
use crate::services::authenticate_token::AuthenticationGuard;
use actix_web::{
    delete, get, patch, post,
    web::{self, Data, Json, Path, Query},
    HttpResponse, Responder,
};
use serde_json::json;

#[post("")]
async fn new_slide(
    body: Json<CreateSlideBody>,
    auth_guard: AuthenticationGuard,
    data: Data<AppState>,
) -> impl Responder {
    let slide_id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query("INSERT INTO slides (id, show_id, content) VALUES (?, ?, ?))")
        .bind(slide_id.clone())
        .bind(body.show_id.to_string())
        .bind(body.content.to_string())
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") && err.contains("'slides.id'") {
            return HttpResponse::BadRequest().json(
                    serde_json::json!({"status": "fail","message": "This id is already associated with a slide"}),
                );
        }
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    }

    match sqlx::query_as!(SlideModelSql, "SELECT * FROM slides WHERE id = ?", slide_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(result) => {
            let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "slide": filter_db_record(&result)
            })});
            return HttpResponse::Ok().json(json_response);
        }
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
        }
    }
}

#[patch("/{id}")]
async fn edit_slide(
    path: Path<uuid::Uuid>,
    body: Json<UpdateSlideBody>,
    auth_guard: AuthenticationGuard,
    data: Data<AppState>,
) -> impl Responder {
    let slide_id = path.into_inner().to_string();

    match sqlx::query("UPDATE slides SET content = ? WHERE id = ?")
        .bind(body.content.to_owned())
        .bind(slide_id.to_owned())
        .execute(&data.db)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let json_response = serde_json::json!({ "status": "error","message": format!("Slide with ID: {} not found", slide_id)});
                return HttpResponse::NotFound().json(json!(json_response));
            }
        }
        Err(err) => {
            let json_response =
                serde_json::json!({ "status": "error","message": format!("{:?}", err) });
            return HttpResponse::InternalServerError().json(json!(json_response));
        }
    }

    match sqlx::query_as!(SlideModelSql, "SELECT * FROM slides WHERE id = ?", slide_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(result) => {
            let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "slide": &result
            })});
            return HttpResponse::Ok().json(json_response);
        }
        Err(sqlx::Error::RowNotFound) => {
            let json_response = serde_json::json!({"status": "fail","message": format!("Slide with id: {} not found", slide_id)});
            return HttpResponse::NotFound().json(json!(json_response));
        }
        Err(err) => {
            let json_response =
                serde_json::json!({ "status": "error","message": format!("{:?}", err) });
            return HttpResponse::InternalServerError().json(json!(json_response));
        }
    };
}

#[delete("/{id}")]
async fn delete_slide(
    path: Path<String>,
    params: Query<DeleteSlideParams>,
    auth_guard: AuthenticationGuard,
    data: Data<AppState>,
) -> impl Responder {
    let slide_id = path.into_inner().to_string();

    match sqlx::query!("DELETE FROM slides WHERE id = ?", slide_id,)
        .execute(&data.db)
        .await
    {
        Ok(slide) => {
            if slide.rows_affected() == 0 {
                let json_response = serde_json::json!({ "status": "fail","message": format!("slide with ID: {} not found", slide_id) });
                return HttpResponse::NotFound().json(json_response);
            } else {
                return HttpResponse::NoContent().finish();
            }
        }
        Err(err) => {
            let json_response =
                serde_json::json!({ "status": "error","message": format!("{:?}", err) });
            return HttpResponse::InternalServerError().json(json_response);
        }
    };
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/slides")
        // .service(get_all_shows)
        // .service(get_user_shows)
        // .service(get_show_by_id)
        .service(new_slide)
        .service(edit_slide)
        .service(delete_slide);
    conf.service(scope);
}
