use crate::models::{
    app::AppState,
    auth::{QueryCode, TokenClaims},
};
use crate::services::github_auth::{get_github_user, request_token};

use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpResponse, Responder,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;

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

// #[get("/logout")]
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
        .service(get_login_url)
        // .service(logout_handler)
        .service(github_oauth_handler);
    conf.service(scope);
}
