use crate::config;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;

pub struct AppState {
    pub db: MySqlPool,
    pub env: config::Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}

impl AppState {
    pub fn init(pool: &MySqlPool) -> AppState {
        AppState {
            db: pool.clone(),
            env: config::Config::init(),
        }
    }
}
