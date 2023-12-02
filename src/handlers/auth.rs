use crate::models::{app::AppState, auth::QueryCode};
use crate::services::{authenticate_token::AuthenticationGuard, github_auth::request_token};
use actix_session::Session;
use actix_web::{
    get,
    web::{scope, Data, Query, ServiceConfig},
    HttpResponse, Responder,
};
use reqwest::header::LOCATION;

#[get("/login")]
async fn get_login_url(data: Data<AppState>) -> impl Responder {
    let client_id = data.env.github_oauth_client_id.to_owned();
    let redirect_url = data.env.github_oauth_redirect_url.to_owned();
    let state = uuid::Uuid::new_v4().to_string();

    let root_url = "https://github.com/login/oauth/authorize";
    let params = [
        ("client_id", client_id.as_str()),
        ("redirect_uri", redirect_url.as_str()),
        ("scope", "read:user"),
        ("state", state.as_str()),
        ("prompt", "consent"),
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
    HttpResponse::Ok().json(json_response)
}

#[get("/oauth/github")]
async fn github_oauth_handler(
    query: Query<QueryCode>,
    data: Data<AppState>,
    session: Session,
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
    session
        .insert("access_token", token_response.access_token)
        .unwrap_or_default();

    let frontend_origin = &data.env.client_origin;
    HttpResponse::Found()
        .append_header((LOCATION, format!("{}/profile", frontend_origin)))
        .finish()
}

#[get("/current_user")]
async fn get_current_user(auth_guard: AuthenticationGuard) -> impl Responder {
    let user = auth_guard.user;
    let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": user,
    })});

    HttpResponse::Ok().json(json_response)
}

#[get("/logout")]
async fn logout_handler(session: Session) -> impl Responder {
    session.purge();

    let json_response = serde_json::json!({"status": "success"});
    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/auth")
        .service(get_login_url)
        .service(github_oauth_handler)
        .service(get_current_user)
        .service(logout_handler);

    conf.service(scope);
}
