use crate::models::auth::GitHubUserModel;
use crate::models::{app::AppState, auth::TokenClaims};
use crate::services::github_auth::get_github_user;
use actix_session::{Session, SessionExt};
use actix_web::{
    dev::Payload,
    error::{Error as ActixWebError, ErrorUnauthorized},
    http, web, FromRequest, HttpRequest, HttpResponse,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::json;
use std::error::Error;
use std::future::{
    Future, {ready, Ready},
};
use std::pin::Pin;

pub struct AuthenticationGuard {
    pub user: GitHubUserModel,
}

impl FromRequest for AuthenticationGuard {
    type Error = ActixWebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let session = req.get_session();
            let access_token = session.get::<String>("access_token");

            match access_token {
                Ok(Some(token)) => match get_github_user(&token).await {
                    Ok(user) => Ok(AuthenticationGuard { user }),
                    Err(error) => Err(ErrorUnauthorized(
                        json!({"status": "error", "message": error.to_string()}),
                    )),
                },
                Ok(None) => Err(ErrorUnauthorized(
                    json!({"status": "fail", "message": "You are not logged in"}),
                )),
                Err(error) => Err(ErrorUnauthorized(
                    json!({"status": "error", "message": error.to_string()}),
                )),
            }
        })
    }
}

pub struct PublicAuthenticationGuard {
    pub user: Option<GitHubUserModel>,
}

impl FromRequest for PublicAuthenticationGuard {
    type Error = ActixWebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let session = req.get_session();
            let access_token = session.get::<String>("access_token");

            match access_token {
                Ok(Some(token)) => match get_github_user(&token).await {
                    Ok(user) => Ok(PublicAuthenticationGuard { user: Some(user) }),
                    Err(_) => Ok(PublicAuthenticationGuard { user: None }),
                },
                Ok(None) => Ok(PublicAuthenticationGuard { user: None }),
                Err(_) => Ok(PublicAuthenticationGuard { user: None }),
            }
        })
    }
}
