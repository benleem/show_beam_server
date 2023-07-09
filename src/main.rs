mod handlers;
use handlers::{auth, shows};
mod config;
mod models;
mod services;

use actix_cors::Cors;
use actix_web::{
    get, http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use sqlx::mysql::MySqlPoolOptions;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info"); // logging api activity, good for dev
    dotenv().ok();
    env_logger::init(); // logging api activity, good for dev

    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let base_url = std::env::var("BASE_URL").expect("BASE_URL must be set");
    let port = std::env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .unwrap();

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
            .allowed_origin("http://localhost:3000")
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
            .service(get_home)
            .configure(shows::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind((base_url, port))?
    .run()
    .await
}

#[get("/")]
async fn get_home() -> impl Responder {
    let json_response = serde_json::json!({"status": "success","message": "Welcome to the Mark Slide api"
    });
    HttpResponse::Ok().json(json_response)
}
