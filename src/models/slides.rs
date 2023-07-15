use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct SlideModelSql {
    pub id: String,
    pub show_id: String,
    pub owner_id: String,
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
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteSLideParams {
    pub id: String,
    pub owner_id: String,
}
