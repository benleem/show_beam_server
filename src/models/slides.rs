use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct SlideModelSql {
    pub id: String,
    pub show_id: String,
    pub user_id: u32,
    pub index_number: i32,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SlideModel {
    pub id: String,
    pub show_id: String,
    pub user_id: u32,
    pub index_number: i32,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSlideBody {
    pub show_id: String,
    pub content: String,
    pub index_number: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSlideBody {
    pub content: String,
    pub index_number: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteSlideBody {
    pub show_id: String,
    pub slide_index: i32,
}

pub fn filter_db_record(slide: &SlideModelSql) -> SlideModel {
    SlideModel {
        id: slide.id.to_owned(),
        show_id: slide.show_id.to_owned(),
        user_id: slide.user_id.to_owned(),
        index_number: slide.index_number.to_owned(),
        content: slide.content.to_owned(),
        created_at: slide.created_at,
        updated_at: slide.updated_at,
    }
}
