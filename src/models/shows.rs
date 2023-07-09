use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ShowModel {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    pub description: String,
    pub view_code: Option<String>,
}

// models for what should be recieved in params/body of POST, PUT, GET, DELETE request hitting the /shows endpoint

#[derive(Debug, Deserialize)]
pub struct GetUserShowsParams {
    pub favorites: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateShowBody {
    pub owner_id: String,
    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateShowBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteShowParams {
    pub owner_id: String,
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
