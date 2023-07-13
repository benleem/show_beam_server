use crate::models::{
    app::AppState,
    auth::{QueryCode, TokenClaims},
    users::UserModel,
};
use crate::services::{
    authenticate_token::AuthenticationGuard,
    github_auth::{get_github_user, request_token},
};

use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get,
    web::{scope, Data, Query, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
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
async fn github_oauth_handler(query: Query<QueryCode>, data: Data<AppState>) -> impl Responder {
    let frontend_origin = data.env.client_origin.to_owned();
    let code = &query.code;
    // let state = &query.state;

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

    let create_user_query = sqlx::query(
        "INSERT INTO users (id, name, username, email, avatar_url, profile_url) VALUES (?, ?, ?, NULLIF(?, ''), ?, ?)",
    )
    .bind(github_user.id.to_string())
    .bind(github_user.name)
    .bind(github_user.login)
    .bind(github_user.email.unwrap_or_default())
    .bind(github_user.avatar_url)
    .bind(github_user.html_url)
    .execute(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = create_user_query {
        // if err.contains("Duplicate entry") && err.contains("'users.PRIMARY'") {
        //     HttpResponse::BadRequest()
        //         .append_header((LOCATION, format!("{}/profile", frontend_origin)))
        //         .json(serde_json::json!({"status": "error","message": "This id is already associated with a user"}));
        // }
        HttpResponse::InternalServerError()
            .append_header((LOCATION, format!("{}/profile", frontend_origin)))
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    }

    let jwt_secret = data.env.jwt_secret.to_owned();
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(data.env.jwt_max_age)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: github_user.id.to_string(),
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
        .domain("localhost")
        .path("/")
        .max_age(ActixWebDuration::new(60 * data.env.jwt_max_age, 0))
        .http_only(true)
        // .secure(true) //for production
        .finish();

    let mut response = HttpResponse::Found();
    response.append_header((LOCATION, format!("{}/profile", frontend_origin)));
    response.cookie(cookie);
    response.finish()
}

#[get("/current_user")]
async fn get_current_user(auth_guard: AuthenticationGuard, data: Data<AppState>) -> impl Responder {
    let user_id = auth_guard.user_id.to_owned();

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

#[get("/logout")]
async fn logout_handler(_: AuthenticationGuard) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .domain("localhost")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(serde_json::json!({"status": "success"}))
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/auth")
        .service(get_login_url)
        .service(github_oauth_handler)
        .service(get_current_user)
        .service(logout_handler);
    conf.service(scope);
}
