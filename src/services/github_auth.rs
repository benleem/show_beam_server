use crate::models::{
    app::AppState,
    auth::{GitHubUserModel, OAuthResponse},
};

use actix_web::web;
use reqwest::Client;
use std::error::Error;

pub async fn request_token(
    authorization_code: &str,
    data: &web::Data<AppState>,
) -> Result<OAuthResponse, Box<dyn Error>> {
    let client_secret = data.env.github_oauth_client_secret.to_owned();
    let client_id = data.env.github_oauth_client_id.to_owned();

    let root_url = "https://github.com/login/oauth/access_token";
    let client = Client::new();
    let params = [
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("code", authorization_code),
    ];
    let response = client
        .post(root_url)
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let oauth_response = response.json::<OAuthResponse>().await?;
        Ok(oauth_response)
    } else {
        let message = "An error occurred while trying to retrieve access token.";
        Err(From::from(message))
    }
}

pub async fn get_github_user(access_token: &str) -> Result<GitHubUserModel, Box<dyn Error>> {
    let root_url = "https://api.github.com/user";
    let client = Client::new();

    let response = client
        .get(root_url)
        .header("Accept", "application/json")
        .header("User-Agent", "Rust")
        .bearer_auth(access_token)
        .send()
        .await?;

    if response.status().is_success() {
        let user_info = response.json::<GitHubUserModel>().await?;
        Ok(user_info)
    } else {
        let message = "An error occurred while trying to retrieve user information.";
        Err(From::from(message))
    }
}
