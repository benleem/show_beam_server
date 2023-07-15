use crate::models::{
    app::AppState,
    shows::{
        filter_db_record, CreateShowBody, DeleteShowParams, GetUserShowsParams, ShowModel,
        ShowModelSql, UpdateShowBody,
    },
};
use crate::services::authenticate_token::AuthenticationGuard;
use actix_web::{
    delete, get, patch, post,
    web::{self, Data, Json, Path, Query},
    HttpResponse, Responder,
};
use serde_json::json;

#[get("")]
async fn get_all_shows(data: Data<AppState>) -> impl Responder {
    // need to implement pagination
    match sqlx::query_as!(ShowModelSql, "SELECT * FROM shows")
        .fetch_all(&data.db)
        .await
    {
        Ok(result) => {
            let shows = result
                .into_iter()
                .map(|show| filter_db_record(&show))
                .collect::<Vec<ShowModel>>();
            let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "shows": shows
            })});
            return HttpResponse::Ok().json(json_response);
        }
        Err(err) => {
            let json_response =
                serde_json::json!({ "status": "error","message": format!("{:?}", err) });
            return HttpResponse::InternalServerError().json(json!(json_response));
        }
    };
}

#[get("/{id}")]
async fn get_show_by_id(path: Path<String>, data: Data<AppState>) -> impl Responder {
    let show_id = path.into_inner().to_string();

    match sqlx::query_as!(ShowModelSql, "SELECT * FROM shows WHERE id = ?", show_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(result) => {
            let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "show": filter_db_record(&result)
            })});
            return HttpResponse::Ok().json(json_response);
        }
        Err(sqlx::Error::RowNotFound) => {
            let json_response = serde_json::json!({"status": "fail","message": format!("Show with id: {} not found", show_id)});
            return HttpResponse::NotFound().json(json!(json_response));
        }
        Err(err) => {
            let json_response =
                serde_json::json!({ "status": "error","message": format!("{:?}", err) });
            return HttpResponse::InternalServerError().json(json!(json_response));
        }
    };
}

#[get("/users/{id}")]
async fn get_user_shows(
    path: Path<String>,
    params: Query<GetUserShowsParams>,
    data: Data<AppState>,
) -> impl Responder {
    let favorites = params.favorites;
    let user_id = path.into_inner().to_string();

    let query_result = match favorites {
    true => sqlx::query_as::<_, ShowModelSql>("SELECT * FROM shows WHERE owner_id = ?")
        .bind(&user_id)
        .fetch_all(&data.db)
        .await,
    false => sqlx::query_as::<_, ShowModelSql>(
        "SELECT shows.* FROM shows INNER JOIN favorites ON shows.owner_id = favorites.owner_id WHERE favorites.owner_id = ?"
    )
        .bind(&user_id)
        .fetch_all(&data.db)
        .await,
    };

    match query_result {
        Ok(result) => {
            if result.len() == 0 {
                let json_response = serde_json::json!({ "status": "error","message": format!("No shows are associated with this user: {}", user_id)});
                return HttpResponse::NotFound().json(json!(json_response));
            }
            let shows = result
                .into_iter()
                .map(|show| filter_db_record(&show))
                .collect::<Vec<ShowModel>>();
            let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "shows": shows
            })});
            return HttpResponse::Ok().json(json_response);
        }
        Err(err) => {
            let json_response =
                serde_json::json!({ "status": "error","message": format!("{:?}", err) });
            return HttpResponse::InternalServerError().json(json_response);
        }
    };
}

#[post("")]
async fn new_show(
    body: Json<CreateShowBody>,
    auth_guard: AuthenticationGuard,
    data: Data<AppState>,
) -> impl Responder {
    let show_id = uuid::Uuid::new_v4().to_string();
    let user_id = auth_guard.user_id.to_owned();

    let query_result = sqlx::query(
        "INSERT INTO shows (id, owner_id, title, description, public, view_code) VALUES (?, ?, ?, ?, ?, NULLIF(?, ''))",
    )
    .bind(show_id.clone())
    .bind(user_id.to_string())
    .bind(body.title.to_string())
    .bind(body.description.to_string())
    .bind(body.public)
    .bind(body.view_code.to_owned().unwrap_or_default())
    .execute(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") && err.contains("'shows.view_code'") {
            return HttpResponse::BadRequest().json(
                serde_json::json!({"status": "fail","message": "This view code has been taken"}),
            );
        }
        if err.contains("Duplicate entry") && err.contains("'shows.id'") {
            return HttpResponse::BadRequest().json(
                    serde_json::json!({"status": "fail","message": "This id is already associated with a show"}),
                );
        }
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    }

    match sqlx::query_as!(ShowModelSql, "SELECT * FROM shows WHERE id = ?", show_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(result) => {
            let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "show": filter_db_record(&result)
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
async fn edit_show(
    path: Path<uuid::Uuid>,
    body: Json<UpdateShowBody>,
    auth_guard: AuthenticationGuard,
    data: Data<AppState>,
) -> impl Responder {
    let show_id = path.into_inner().to_string();
    let user_id = auth_guard.user_id.to_owned();

    match sqlx::query(
        "UPDATE shows SET title = COALESCE(NULLIF(?, ''), title), description = COALESCE(NULLIF(?, ''), description), public = COALESCE(NULLIF(?, ''), public), view_code = COALESCE(NULLIF(?, ''), view_code) WHERE id = ? AND owner_id = ?",
    )
    .bind(body.title.to_owned().unwrap_or_default())
    .bind(body.description.to_owned().unwrap_or_default())
    .bind(body.public.to_owned().unwrap_or_default())
    .bind(body.view_code.to_owned().unwrap_or_default())
    .bind(show_id.to_owned())
    .bind(user_id)
    .execute(&data.db)
    .await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let json_response = serde_json::json!({ "status": "error","message": format!("Show with ID: {} not found", show_id)});
                return HttpResponse::NotFound().json(json!(json_response));
            }
        }
        Err(err) => {
            if err.to_string().contains("Duplicate entry")
                && err.to_string().contains("'shows.view_code'")
            {
                let json_response = serde_json::json!({ "status": "error","message": "This view code has been taken"});
                return HttpResponse::InternalServerError().json(json!(json_response));
            }
            let json_response =
                serde_json::json!({ "status": "error","message": format!("{:?}", err) });
            return HttpResponse::InternalServerError().json(json!(json_response));
        }
    }

    match sqlx::query_as!(ShowModelSql, "SELECT * FROM shows WHERE id = ?", show_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(result) => {
            let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "show": filter_db_record(&result)
            })});
            return HttpResponse::Ok().json(json_response);
        }
        Err(sqlx::Error::RowNotFound) => {
            let json_response = serde_json::json!({"status": "fail","message": format!("Show with id: {} not found", show_id)});
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
async fn delete_show(
    path: Path<String>,
    // params: Query<DeleteShowParams>,
    auth_guard: AuthenticationGuard,
    data: Data<AppState>,
) -> impl Responder {
    let show_id = path.into_inner().to_string();
    let user_id = auth_guard.user_id.to_owned();

    match sqlx::query!(
        "DELETE FROM shows WHERE id = ? AND owner_id = ?",
        show_id,
        user_id
    )
    .execute(&data.db)
    .await
    {
        Ok(show) => {
            if show.rows_affected() == 0 {
                let json_response = serde_json::json!({ "status": "fail","message": format!("Show with ID: {} not found", show_id) });
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
    let scope = web::scope("/shows")
        .service(get_all_shows)
        .service(get_user_shows)
        .service(get_show_by_id)
        .service(new_show)
        .service(edit_show)
        .service(delete_show);
    conf.service(scope);
}
