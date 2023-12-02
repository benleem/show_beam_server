use crate::config;
// use secret::Secret;
use sqlx::mysql::MySqlPool;

pub struct AppState {
    pub db: MySqlPool,
    pub env: config::Config,
    // access_token: Option<Secret<String>>,
}

impl AppState {
    pub fn init(pool: &MySqlPool) -> AppState {
        AppState {
            db: pool.clone(),
            env: config::Config::init(),
            // access_token: None,
        }
    }
}
