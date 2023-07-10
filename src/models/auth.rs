use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubUserModel {
    pub login: String,
    pub id: usize,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub site_admin: bool,
    pub name: String,
    pub company: Option<String>,
    pub blog: String,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: usize,
    pub public_gists: usize,
    pub followers: usize,
    pub following: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub private_gists: usize,
    pub total_private_repos: usize,
    pub owned_private_repos: usize,
    pub disk_usage: usize,
    pub collaborators: usize,
    pub two_factor_authentication: bool,
    pub plan: GitHubUserPlan,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubUserPlan {
    pub name: String,
    pub space: usize,
    pub collaborators: usize,
    pub private_repos: usize,
}
