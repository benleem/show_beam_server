use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct FavoriteModelSql {
    pub id: String,
    pub show_id: String,
    pub user_id: u32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FavoriteModel {
    pub id: String,
    pub show_id: String,
    pub user_id: u32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetFavoritesParams {
    pub user_id: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFavoriteBody {
    pub show_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteFavoriteParams {
    pub show_id: String,
    pub user_id: u32,
}

pub fn filter_db_record(show: &FavoriteModelSql) -> FavoriteModel {
    FavoriteModel {
        id: show.id.to_owned(),
        show_id: show.show_id.to_owned(),
        user_id: show.user_id.to_owned(),
        created_at: show.created_at,
        updated_at: show.updated_at,
    }
}
