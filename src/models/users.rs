use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserModel {
    pub username: String,
    pub id: u32,
    pub avatar_url: String,
    pub profile_url: String,
    pub name: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// models for what should be recieved in params/body of POST, PUT, GET, DELETE request hitting the /users endpoint

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteUserParam {
    pub id: String,
}

// response and request structs for hitting the /users endpoint

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserModel,
}
