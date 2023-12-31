#[derive(Debug, Clone)]
pub struct Config {
    pub cookie_domain: String,
    pub client_origin: String,
    pub jwt_secret: String,
    pub secret_key: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: i64,
    pub github_oauth_client_id: String,
    pub github_oauth_client_secret: String,
    pub github_oauth_redirect_url: String,
}

impl Config {
    pub fn init() -> Config {
        let cookie_domain = std::env::var("COOKIE_DOMAIN").expect("COOKIE_DOMAIN must be set");
        let client_origin = std::env::var("CLIENT_ORIGIN").expect("CLIENT_ORIGIN must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let jwt_expires_in =
            std::env::var("TOKEN_EXPIRED_IN").expect("TOKEN_EXPIRED_IN must be set");
        let jwt_max_age = std::env::var("TOKEN_MAXAGE").expect("TOKEN_MAXAGE must be set");
        let github_oauth_client_id =
            std::env::var("GITHUB_OAUTH_CLIENT_ID").expect("GITHUB_OAUTH_CLIENT_ID must be set");
        let github_oauth_client_secret = std::env::var("GITHUB_OAUTH_CLIENT_SECRET")
            .expect("GITHUB_OAUTH_CLIENT_SECRET must be set");
        let github_oauth_redirect_url = std::env::var("GITHUB_OAUTH_REDIRECT_URL")
            .expect("GITHUB_OAUTH_REDIRECT_URL must be set");

        Config {
            cookie_domain,
            client_origin,
            jwt_secret,
            secret_key,
            jwt_expires_in,
            jwt_max_age: jwt_max_age.parse::<i64>().unwrap(),
            github_oauth_client_id,
            github_oauth_client_secret,
            github_oauth_redirect_url,
        }
    }
}
