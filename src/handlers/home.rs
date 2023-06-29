use actix_web::{
    get,
    web::{scope, ServiceConfig},
    HttpResponse, Responder,
};

// endpoints

#[get("")]
async fn get_home() -> impl Responder {
    let json_response = serde_json::json!({"status": "success","message": "Welcome to the Mark Slide api"
    });
    return HttpResponse::Ok().json(json_response);
}

// config

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/").service(get_home);
    conf.service(scope);
}
