use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Show {
    id: String,
    owner: String,
    title: String,
    description: String,
    view_code: String,
}

// models for what should be recieved in body of POST, PUT request hitting the /shows endpoint

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateShow {
    id: String,
    owner: String,
    title: String,
    description: String,
    view_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateShow {
    title: Option<String>,
    description: Option<String>,
    view_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteShow {
    id: String,
    owner: String,
}

// response models for requests hitting the /shows endpoint

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ShowResponse {
    id: String,
    owner: String,
    title: String,
    description: String,
    viewCode: String,
}
