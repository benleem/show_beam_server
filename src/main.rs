#![allow(unused)]
use actix_web::{web, App, HttpServer, Result};
use sqlx::Connection;
use std::env;
#[actix_web::main]
pub async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = sqlx::MySqlConnection::connect(&url).await.unwrap();
    println!("{:?}", url);
    Ok(())
}
