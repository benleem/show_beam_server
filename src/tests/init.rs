use crate::models::app::AppState;
use actix_http::Request;
use actix_service::Service;
use actix_web::{
    dev::{HttpServiceFactory, ServiceResponse},
    test, web, App, Error,
};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

#[allow(dead_code)]
pub async fn init(
    scope: &str,
    service_factory: impl HttpServiceFactory + 'static,
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    test::init_service(
        App::new()
            .app_data(web::Data::new(AppState::init(&get_pool().await)))
            .service(web::scope(scope).service(service_factory)),
    )
    .await
}

async fn get_pool() -> MySqlPool {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;
    pool.unwrap()
}
