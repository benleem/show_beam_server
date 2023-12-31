use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ShowModelSql {
    pub id: String,
    pub user_id: u32,
    pub title: String,
    pub description: String,
    pub public: i8, //sql doesn't have bool :(
    pub view_code: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShowModel {
    pub id: String,
    pub user_id: u32,
    pub title: String,
    pub description: String,
    pub public: bool,
    pub view_code: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShowUrlQueryParams {
    pub view_code: Option<String>,
}

// models for what should be recieved in params/body of POST, PUT, GET, DELETE request hitting the /shows endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserShowsParams {
    pub favorites: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateShowBody {
    pub title: String,
    pub description: String,
    pub public: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateShowBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    pub view_code: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteShowParams {
    pub user_id: u32,
}

// response and request structs for hitting the /shows endpoint

// #[derive(Debug, Deserialize, Serialize)]
// #[allow(non_snake_case)]
// pub struct ShowResponse {
//     pub id: String,
//     pub ownerId: String,
//     pub title: String,
//     pub description: String,
//     pub viewCode: String,
// }

// convert MySql i8 into Rust bool
pub fn filter_db_record(show: &ShowModelSql) -> ShowModel {
    ShowModel {
        id: show.id.to_owned(),
        user_id: show.user_id.to_owned(),
        title: show.title.to_owned(),
        description: show.description.to_owned(),
        public: show.public != 0,
        view_code: show.view_code.to_owned(),
        created_at: show.created_at,
        updated_at: show.updated_at,
    }
}
