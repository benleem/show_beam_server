use crate::models::{
    app::AppState,
    auth::{QueryCode, TokenClaims},
    users::{RegisterUserParams, UserData, UserModel, UserResponse},
};
use crate::services::github_auth::{get_github_user, request_token};

use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpResponse, Responder,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

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

#[get("/login")]
async fn get_login_url(data: web::Data<AppState>) -> impl Responder {
    let client_id = data.env.github_oauth_client_id.to_owned();
    let redirect_url = data.env.github_oauth_redirect_url.to_owned();
    let state = uuid::Uuid::new_v4().to_string();

    let root_url = "https://github.com/login/oauth/authorize";
    let params = [
        ("client_id", client_id.as_str()),
        ("redirect_uri", redirect_url.as_str()),
        ("scope", "read:user"),
        ("state", state.as_str()),
    ];

    let mut login_url = root_url.to_string();
    for (i, param) in params.iter().enumerate() {
        let key = param.0;
        let value = param.1;

        if i == 0 {
            login_url.push_str("?");
        } else {
            login_url.push_str("&");
        }

        login_url.push_str(key);
        login_url.push_str("=");
        login_url.push_str(value);
    }

    let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "login_url": login_url
    })});
    return HttpResponse::Ok().json(json_response);
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

#[get("/oauth/github")]
async fn github_oauth_handler(
    query: web::Query<QueryCode>,
    data: web::Data<AppState>,
) -> impl Responder {
    let code = &query.code;
    let state = &query.state;

    let token_response = request_token(code.as_str(), &data).await;
    if token_response.is_err() {
        let message = token_response.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let token_response = token_response.unwrap();

    let github_user = get_github_user(&token_response.access_token).await;
    if github_user.is_err() {
        let message = github_user.err().unwrap().to_string();
        return HttpResponse::BadGateway()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }
    let github_user = github_user.unwrap();

    let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "github_user":github_user
    })});
    return HttpResponse::Ok().json(json_response);
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        // .service(register_user_handler)
        .service(get_login_url)
        // .service(login_user_handler)
        .service(github_oauth_handler);
    conf.service(scope);
}
