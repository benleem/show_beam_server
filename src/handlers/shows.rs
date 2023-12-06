use crate::services::authenticate_token::AuthenticationGuard;
use crate::{
    models::{
        app::AppState,
        shows::{
            filter_db_record, CreateShowBody, DeleteShowParams, GetUserShowsParams, ShowModel,
            ShowModelSql, ShowUrlQueryParams, UpdateShowBody,
        },
    },
    services::authenticate_token::PublicAuthenticationGuard,
};
use actix_session::Session;
use actix_web::{
    delete, get, patch, post,
    web::{self, Data, Json, Path, Query},
    HttpResponse, Responder,
};
use chrono::ParseMonthError;
use serde_json::json;

#[get("")]
async fn get_all_shows(data: Data<AppState>) -> impl Responder {
    // need to implement pagination
    match sqlx::query_as!(
        ShowModelSql,
        "SELECT * FROM shows where public = 1 ORDER BY updated_at DESC"
    )
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
async fn get_show_by_id(
    data: Data<AppState>,
    path: Path<String>,
    params: Query<ShowUrlQueryParams>,
    auth_guard: PublicAuthenticationGuard,
) -> impl Responder {
    let show_id = path.into_inner().to_string();
    let view_code = params.view_code.clone().unwrap_or_default();

    match sqlx::query_as!(ShowModelSql, "SELECT * FROM shows WHERE id = ?", &show_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(result) => {
            let show = filter_db_record(&result);

            // return show if show is public or view code is passed in params
            fn check_alternate_auth(show: ShowModel, view_code: String) -> HttpResponse {
                if show.public || view_code == show.view_code {
                    let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                        "show": show,
                        "can_edit": false
                    })});
                    return HttpResponse::Ok().json(json_response);
                }
                let json_response = serde_json::json!({"status": "fail","message": "You are not authorized to view this show"});
                return HttpResponse::Unauthorized().json(json_response);
            }

            match auth_guard.user {
                Some(user) => {
                    if user.id as u32 == show.user_id {
                        // return show if user is owner
                        let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                            "show": show,
                            "can_edit": true
                        })});
                        return HttpResponse::Ok().json(json_response);
                    } else {
                        // return show if user is editor
                        match sqlx::query_as::<_, ShowModelSql>(
                            "SELECT * FROM editors WHERE user_id = ? AND show_id = ?",
                        )
                        .bind(user.id.to_string())
                        .bind(&show_id)
                        .fetch_one(&data.db)
                        .await
                        {
                            Ok(_) => {
                                let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                                    "show": show,
                                    "can_edit": true
                                })});
                                return HttpResponse::Ok().json(json_response);
                            }
                            Err(_) => {
                                return check_alternate_auth(show, view_code);
                            }
                        };
                    }
                }
                None => {
                    return check_alternate_auth(show, view_code);
                }
            };
        }
        Err(sqlx::Error::RowNotFound) => {
            let json_response = serde_json::json!({"status": "fail","message": format!("Show with id: {} not found", show_id)});
            return HttpResponse::NotFound().json(json_response);
        }
        Err(err) => {
            let json_response =
                serde_json::json!({ "status": "error","message": format!("{:?}", err) });
            return HttpResponse::InternalServerError().json(json_response);
        }
    };
}

#[get("/users/{id}")]
async fn get_user_shows(
    auth_guard: AuthenticationGuard,
    path: Path<String>,
    params: Query<GetUserShowsParams>,
    data: Data<AppState>,
) -> impl Responder {
    let favorites = params.favorites;
    let user_id = path.into_inner().to_string();

    let query_result = match favorites {
    false => sqlx::query_as::<_, ShowModelSql>("SELECT * FROM shows WHERE user_id = ? ORDER BY updated_at DESC")
        .bind(&user_id)
        .fetch_all(&data.db)
        .await,
    true => sqlx::query_as::<_, ShowModelSql>(
        "SELECT * FROM shows INNER JOIN favorites ON shows.user_id = favorites.user_id WHERE favorites.user_id = ? ORDER BY updated_at DESC"
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
    let user_id = auth_guard.user.id.to_string();
    let view_code = uuid::Uuid::new_v4().to_string();

    let query_result = sqlx::query(
        "INSERT INTO shows (id, user_id, title, description, public, view_code) VALUES (?, ?, ?, ?, ?, NULLIF(?, ''))",
    )
    .bind(&show_id)
    .bind(user_id)
    .bind(body.title.to_string())
    .bind(body.description.to_string())
    .bind(body.public)
    .bind(view_code)
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
    let user_id = auth_guard.user.id.to_string();
    let view_code = match body.view_code {
        true => Some(uuid::Uuid::new_v4().to_string()),
        false => None,
    };

    match sqlx::query(
        "UPDATE shows SET title = COALESCE(NULLIF(?, ''), title), description = COALESCE(NULLIF(?, ''), description), public = COALESCE(NULLIF(?, NULL), public), view_code = COALESCE(NULLIF(?, ''), view_code) WHERE id = ? AND user_id = ?",
    )
    .bind(body.title.to_owned().unwrap_or_default())
    .bind(body.description.to_owned().unwrap_or_default())
    .bind(body.public.to_owned().unwrap_or_default())
    .bind(view_code.unwrap_or_default())
    .bind(&show_id)
    .bind(user_id)
    .execute(&data.db)
    .await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                let json_response = serde_json::json!({ "status": "error","message": format!("This show is not associated with the current user")});
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
    let user_id = auth_guard.user.id.to_string();

    let query_result = sqlx::query!(
        "DELETE FROM slides WHERE show_id = ? AND user_id = ?",
        show_id,
        user_id
    )
    .execute(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        // if err.contains("Duplicate entry") && err.contains("'user.id'") {
        //     return HttpResponse::BadRequest().json(
        //             serde_json::json!({"status": "fail","message": ""}),
        //         );
        // }
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    }

    match sqlx::query!(
        "DELETE FROM shows WHERE id = ? AND user_id = ?",
        show_id,
        user_id
    )
    .execute(&data.db)
    .await
    {
        Ok(show) => {
            if show.rows_affected() == 0 {
                let json_response = serde_json::json!({ "status": "fail","message": format!("This show is not associated with the current user") });
                return HttpResponse::NotFound().json(json_response);
            } else {
                let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                    "show": show_id
                })});
                return HttpResponse::Ok().json(json_response);
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
