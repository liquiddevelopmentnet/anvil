use actix_web::*;
use serde_json::json;
use crate::utils;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "version": utils::build_version_string(),
    }))
}
