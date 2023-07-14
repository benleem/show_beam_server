use crate::models::{app::AppState, users::UserModel};

use actix_web::{
    delete, get, patch, post,
    web::{self, Data, Json, Path, Query},
    HttpResponse, Responder,
};

use serde_json::json;
use sqlx;

#[get("")]
async fn get_all_users(data: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(UserModel, "SELECT * FROM users")
        .fetch_all(&data.db)
        .await
    {
        Ok(result) => {
            let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "users": result
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
async fn get_user_by_id(path: Path<String>, data: Data<AppState>) -> impl Responder {
    let user_id = path.into_inner().to_string();

    match sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = ?", user_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(user) => {
            let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": user,
            })});
            HttpResponse::Ok().json(json_response)
        }
        Err(error) => {
            let json_response =
                serde_json::json!({"status": "error","message": format!("{:?}", error)});
            HttpResponse::InternalServerError().json(json_response)
        }
    }
}

// #[post("")]
// async fn new_user(body: Json<CreateUserBody>, data: Data<AppState>) -> impl Responder {
//     let user_id = uuid::Uuid::new_v4().to_string();

//     let query_result = sqlx::query("INSERT INTO users (id,name,email) VALUES (?, ?, ?)")
//         .bind(user_id.clone())
//         .bind(body.name.to_string())
//         .bind(body.email.to_string())
//         .execute(&data.db)
//         .await
//         .map_err(|err: sqlx::Error| err.to_string());

//     if let Err(err) = query_result {
//         if err.contains("Duplicate entry") && err.contains("'user.id'") {
//             return HttpResponse::BadRequest().json(
//                     serde_json::json!({"status": "fail","message": "This id is already associated with a user"}),
//                 );
//         }
//         return HttpResponse::InternalServerError()
//             .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
//     }

//     match sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = ?", user_id)
//         .fetch_one(&data.db)
//         .await
//     {
//         Ok(result) => {
//             let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
//                 "user": result
//             })});

//             return HttpResponse::Ok().json(json_response);
//         }
//         Err(err) => {
//             return HttpResponse::InternalServerError()
//                 .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
//         }
//     }
// }

// #[patch("/{id}")]
// async fn edit_user(
//     path: web::Path<(String,)>,
//     body: Json<UpdateUserBody>,
//     data: Data<AppState>,
// ) -> impl Responder {
//     match sqlx::query(
//         "UPDATE users SET name = COALESCE(NULLIF(?, ''), name), email = COALESCE(NULLIF(?, ''), email) WHERE id = ?",
//     )
//     .bind(body.name.to_owned().unwrap_or_default())
//     .bind(body.email.to_owned().unwrap_or_default())
//     .bind(&path.0.to_owned())
//     .execute(&data.db)
//     .await {
//         Ok(result) => {
//             if result.rows_affected() == 0 {
//                 let json_response = serde_json::json!({ "status": "error","message": format!("User with ID: {} not found", &path.0)});
//                 return HttpResponse::NotFound().json(json!(json_response));
//             }
//         }
//         Err(err) => {
//             let json_response =
//                 serde_json::json!({ "status": "error","message": format!("{:?}", err) });
//             return HttpResponse::InternalServerError().json(json!(json_response));
//         }
//     };

//     match sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = ?", &path.0.to_string())
//         .fetch_one(&data.db)
//         .await
//     {
//         Ok(result) => {
//             let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
//                 "user": result
//             })});
//             return HttpResponse::Ok().json(json_response);
//         }
//         Err(sqlx::Error::RowNotFound) => {
//             let json_response = serde_json::json!({"status": "fail","message": format!("User with id: {} not found", &path.0)});
//             return HttpResponse::NotFound().json(json!(json_response));
//         }
//         Err(err) => {
//             let json_response =
//                 serde_json::json!({ "status": "error","message": format!("{:?}", err) });
//             return HttpResponse::InternalServerError().json(json!(json_response));
//         }
//     };
// }

#[delete("/{id}")]
async fn delete_user(path: web::Path<String>, data: Data<AppState>) -> impl Responder {
    let user_id = path.into_inner().to_string();

    match sqlx::query!("DELETE FROM users WHERE id = ?", user_id)
        .execute(&data.db)
        .await
    {
        Ok(user) => {
            if user.rows_affected() == 0 {
                let json_response = serde_json::json!({ "status": "fail","message": format!("User with ID: {} not found", user_id) });
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
    let scope = web::scope("/users")
        .service(get_all_users)
        .service(get_user_by_id)
        // .service(new_user)
        // .service(edit_user)
        .service(delete_user);

    conf.service(scope);
}
