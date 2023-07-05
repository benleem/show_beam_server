use crate::config;
use sqlx::mysql::MySqlPool;

pub struct AppState {
    pub db: MySqlPool,
    pub env: config::Config,
}

impl AppState {
    pub fn init(pool: &MySqlPool) -> AppState {
        AppState {
            db: pool.clone(),
            env: config::Config::init(),
        }
    }
}
