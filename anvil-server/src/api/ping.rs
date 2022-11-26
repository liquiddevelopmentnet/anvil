use actix_web::*;
use serde_json::json;
#[path = "../utils.rs"] mod utils;

#[get("/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "version": utils::build_version_string(),
    }))
}
