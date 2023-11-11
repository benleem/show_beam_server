// use crate::models::{
//     app::AppState,
//     favorites::{
//         filter_db_record, CreateFavoriteBody, DeleteFavoriteParams, FavoriteModel,
//         FavoriteModelSql, GetFavoritesParams,
//     },
// };
// use crate::services::authenticate_token::AuthenticationGuard;
// use actix_web::{
//     delete, get, post,
//     web::{self, Data, Json, Path, Query},
//     HttpResponse, Responder,
// };
// use serde_json::json;

// #[get("")]
// async fn get_favorites(
//     body: Json<GetFavoritesParams>,
//     auth_guard: AuthenticationGuard,
//     data: Data<AppState>,
// ) -> impl Responder {
//     match body.user_id {
//         Some(uid) => {
//             match sqlx::query_as!(
//                 FavoriteModelSql,
//                 "SELECT * FROM favorites WHERE user_id = ?",
//                 uid
//             )
//             .fetch_all(&data.db)
//             .await
//             {
//                 Ok(result) => {
//                     let favorites = result
//                         .into_iter()
//                         .map(|favorite| filter_db_record(&favorite))
//                         .collect::<Vec<FavoriteModel>>();
//                     let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
//                         "favorites": favorites
//                     })});
//                     return HttpResponse::Ok().json(json_response);
//                 }
//                 Err(err) => {
//                     let json_response =
//                         serde_json::json!({ "status": "error","message": format!("{:?}", err) });
//                     return HttpResponse::InternalServerError().json(json!(json_response));
//                 }
//             }
//         }
//         None => {
//             match sqlx::query_as!(FavoriteModelSql, "SELECT * FROM favorites")
//                 .fetch_all(&data.db)
//                 .await
//             {
//                 Ok(result) => {
//                     let favorites = result
//                         .into_iter()
//                         .map(|favorite| filter_db_record(&favorite))
//                         .collect::<Vec<FavoriteModel>>();
//                     let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
//                         "favorites": favorites
//                     })});
//                     return HttpResponse::Ok().json(json_response);
//                 }
//                 Err(err) => {
//                     let json_response =
//                         serde_json::json!({ "status": "error","message": format!("{:?}", err) });
//                     return HttpResponse::InternalServerError().json(json!(json_response));
//                 }
//             }
//         }
//     }
// }

// #[post("")]
// async fn new_favorite(
//     body: Json<CreateFavoriteBody>,
//     auth_guard: AuthenticationGuard,
//     data: Data<AppState>,
// ) -> impl Responder {
//     let favorite_id = uuid::Uuid::new_v4().to_string();
//     let user_id = auth_guard.user_id.to_owned();
//     let query_result =
//         sqlx::query("INSERT INTO favorites (id, show_id, user_id) VALUES (?, ?, ?))")
//             .bind(favorite_id.clone())
//             .bind(body.show_id.to_string())
//             .bind(user_id.to_string())
//             .execute(&data.db)
//             .await
//             .map_err(|err: sqlx::Error| err.to_string());

//     if let Err(err) = query_result {
//         if err.contains("Duplicate entry") && err.contains("'favorites.id'") {
//             return HttpResponse::BadRequest().json(
//                     serde_json::json!({"status": "fail","message": "This id is already associated with a favorite"}),
//                 );
//         }
//         return HttpResponse::InternalServerError()
//             .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
//     }

//     match sqlx::query_as!(
//         FavoriteModelSql,
//         "SELECT * FROM favorites WHERE id = ?",
//         favorite_id
//     )
//     .fetch_one(&data.db)
//     .await
//     {
//         Ok(result) => {
//             let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
//                 "favorite": crate::models::favorites::filter_db_record(&result)
//             })});
//             return HttpResponse::Ok().json(json_response);
//         }
//         Err(err) => {
//             return HttpResponse::InternalServerError()
//                 .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
//         }
//     }
// }

// #[delete("/{id}")]
// async fn delete_favorite(
//     path: Path<String>,
//     params: Query<DeleteFavoriteParams>,
//     auth_guard: AuthenticationGuard,
//     data: Data<AppState>,
// ) -> impl Responder {
//     let favorite_id = path.into_inner().to_string();
//     let user_id = auth_guard.user_id.to_owned();

//     if user_id != params.user_id.to_string() {
//         return HttpResponse::Unauthorized().finish();
//     }

//     match sqlx::query!(
//         "DELETE FROM favorites WHERE id = ? AND user_id = ?",
//         favorite_id,
//         user_id
//     )
//     .execute(&data.db)
//     .await
//     {
//         Ok(favorite) => {
//             if favorite.rows_affected() == 0 {
//                 let json_response = serde_json::json!({ "status": "fail","message": format!("favorite with ID: {} not found", favorite_id) });
//                 return HttpResponse::NotFound().json(json_response);
//             } else {
//                 return HttpResponse::NoContent().finish();
//             }
//         }
//         Err(err) => {
//             let json_response =
//                 serde_json::json!({ "status": "error","message": format!("{:?}", err) });
//             return HttpResponse::InternalServerError().json(json_response);
//         }
//     };
// }

// pub fn config(conf: &mut web::ServiceConfig) {
//     let scope = web::scope("/favorites")
//         .service(get_favorites)
//         .service(new_favorite)
//         .service(delete_favorite);
//     conf.service(scope);
// }
