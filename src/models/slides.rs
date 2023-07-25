use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct SlideModelSql {
    pub id: String,
    pub show_id: String,
    pub user_id: u32,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SlideModel {
    pub id: String,
    pub show_id: String,
    pub user_id: u32,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSlideBody {
    pub show_id: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSlideBody {
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteSlideParams {
    pub user_id: u32,
}

pub fn filter_db_record(show: &SlideModelSql) -> SlideModel {
    SlideModel {
        id: show.id.to_owned(),
        show_id: show.show_id.to_owned(),
        user_id: show.user_id.to_owned(),
        content: show.content.to_owned(),
        created_at: show.created_at,
        updated_at: show.updated_at,
    }
}
