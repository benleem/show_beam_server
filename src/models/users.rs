use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    id: String,
    username: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    id: String,
    username: String,
    email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteUser {
    id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    id: String,
}
