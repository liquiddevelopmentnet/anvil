use actix_web::*;
use serde_json::json;
use crate::*;

#[get("/ping")]
pub async fn get_ping() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "version": utils::version_string(),
    }))
}
