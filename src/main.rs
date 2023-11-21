mod handlers;
use handlers::{auth, favorites, shows, slides};
mod config;
mod models;
mod services;
mod tests;

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::Key, get, http::header, middleware::Logger, web, App, Error, HttpResponse, HttpServer,
    Responder,
};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info"); // logging api activity, good for dev
    dotenv().ok();
    env_logger::init(); // logging api activity, good for dev

    let base_url = std::env::var("BASE_URL").expect("BASE_URL must be set");
    let port = std::env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .unwrap();
    let client_origin = std::env::var("CLIENT_ORIGIN").expect("CLIENT_ORIGIN must be set");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&client_origin)
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(models::app::AppState::init(&pool)))
            .configure(auth::config)
            // .configure(favorites::config)
            .configure(shows::config)
            .configure(slides::config)
            .service(get_home)
            .wrap(cors)
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(Logger::default())
    })
    .bind((base_url, port))?
    .run()
    .await
}

#[get("/")]
async fn get_home(session: Session) -> impl Responder {
    let json_response = serde_json::json!({"status": "success","message": "Welcome to the ShowBeam api"
    });
    HttpResponse::Ok().json(json_response)

    // access session data
    // if let Some(count) = session.get::<i32>("access_token")? {
    //     session.insert("access_token", count + 1)?;
    // } else {
    //     session.insert("access_token", 1)?;
    // }

    // Ok(HttpResponse::Ok().body(format!(
    //     "Count is {:?}!",
    //     session.get::<i32>("counter")?.unwrap()
    // )))
}
