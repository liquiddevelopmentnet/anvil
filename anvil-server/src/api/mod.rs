mod ping;

use actix_web::{HttpResponse, Responder, Scope, web};
use serde_json::json;

pub fn service() -> Scope {
    web::scope("/api")
        .service(
            web::scope("/v1")
                .service(ping::ping)
                .default_service(
                    web::route().to(not_found)
                )
        )
        .default_service(
            web::route().to(invalid_api_version)
        )
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(json!({
        "error": "not_found",
        "message": "the requested endpoint was not found"
    }))
}

pub async fn invalid_api_version() -> impl Responder {
    HttpResponse::BadGateway().json(json!({
        "error": "invalid_api_version",
        "message": "the requested api version is not supported"
    }))
}
