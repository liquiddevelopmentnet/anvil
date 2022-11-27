use actix_web::*;
use serde_json::json;
use crate::*;
use crate::config::projects::Project;

#[get("/{project}/channels")]
pub async fn get_channels(path: web::Path<String>) -> impl Responder {
    let project_id = path.into_inner();
    let project: Option<Project> = config::projects::get().projects.iter().find(|p| p.id == project_id).cloned();
    match project {
        Some(project) => {
            HttpResponse::Ok().json(json!(project.channels))
        },
        None => HttpResponse::NotFound().json(json!({
            "error": "not_found",
            "message": "the requested project was not found"
        }))
    }
}
