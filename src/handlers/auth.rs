use crate::models::{
    auth::{AppState, TokenClaims},
    users::{RegisterUserParams, UserData, UserModel, UserResponse},
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpResponse, Responder,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "How to Implement GitHub OAuth2 in Rust";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[post("/register")]
async fn register_user_handler(
    body: web::Json<RegisterUserParams>,
    data: web::Data<AppState>,
) -> impl Responder {
    match sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE email = ?")
        .bind(&body.email)
        .fetch_optional(&data.db)
        .await
    {
        Ok(response) => match response {
            Some(_) => HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "User already exists"})),
            None => {
                let uuid_id = Uuid::new_v4();
                let user = UserModel {
                    id: uuid_id.to_string(),
                    name: body.name.to_owned(),
                    email: body.email.to_owned(),
                };
                let response = UserResponse {
                    status: "success".to_string(),
                    data: UserData {
                        user: UserModel::user_to_response(&user),
                    },
                };
                HttpResponse::Ok().json(response)
            }
        },
        Err(err) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)})),
    }
}

#[post("/login")]
async fn login_user_handler(
    body: web::Json<RegisterUserParams>,
    data: web::Data<AppState>,
) -> impl Responder {
    match sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE email = ?")
        .bind(&body.email)
        .fetch_optional(&data.db)
        .await
    {
        Ok(user) => {
            let jwt_secret = data.env.jwt_secret.to_owned();
            let now = Utc::now();
            let iat = now.timestamp() as usize;
            let exp = (now + Duration::minutes(data.env.jwt_max_age)).timestamp() as usize;
            let claims: TokenClaims = TokenClaims {
                sub: user.unwrap().id,
                exp,
                iat,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(jwt_secret.as_ref()),
            )
            .unwrap();

            let cookie = Cookie::build("token", token)
                .path("/")
                .max_age(ActixWebDuration::new(60 * data.env.jwt_max_age, 0))
                .http_only(true)
                .finish();

            HttpResponse::Ok()
                .cookie(cookie)
                .json(serde_json::json!({"status": "success"}))
        }
        Err(_) => HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "fail", "message": "Invalid email or password"})),
    }
}

// #[get("/auth/logout")]
// async fn logout_handler(_: AuthenticationGuard) -> impl Responder {
//     let cookie = Cookie::build("token", "")
//         .path("/")
//         .max_age(ActixWebDuration::new(-1, 0))
//         .http_only(true)
//         .finish();
//
//     HttpResponse::Ok()
//         .cookie(cookie)
//         .json(serde_json::json!({"status": "success"}))
// }

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .service(health_checker_handler)
        .service(register_user_handler)
        .service(login_user_handler);
    conf.service(scope);
}
