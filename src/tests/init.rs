use crate::models::app::AppState;
use actix_http::Request;
use actix_service::Service;
use actix_web::{
    dev::{HttpServiceFactory, Payload, ServiceResponse},
    error::Error as ActixWebError,
    test, web, App, FromRequest, HttpRequest,
};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::future::{ready, Ready};

pub struct MockAuthenticationGuard {
    pub user_id: String,
}

impl FromRequest for MockAuthenticationGuard {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(_: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Provide the user_id you want to use for testing
        let user_id = "1001".to_string();
        ready(Ok(MockAuthenticationGuard { user_id }))
    }
}

#[allow(dead_code)]
pub async fn init(
    scope: &str,
    service_factory: impl HttpServiceFactory + 'static,
) -> impl Service<Request, Response = ServiceResponse, Error = ActixWebError> {
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
