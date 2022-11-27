mod ping;
mod channels;

use std::io;
use actix_web::{App, HttpResponse, HttpServer, middleware, Responder, Scope, web};
use serde_json::json;
use crate::*;

pub async fn init() -> io::Result<()> {
    let bind_addr = format!("{}:{}", config::main::get().bind_config.address, config::main::get().bind_config.port);

    let srv = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::DefaultHeaders::new()
                .add(("Server", "anvil"))
                .add(("X-Powered-By", "anvil"))
            )
            .wrap(actix_cors::Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
            )
            .service(service())
    })
        .bind(&bind_addr)?
        .run();

    tokio::join!(
        srv,
        async {
            success!("api server started on http://<cyan>{}</>", bind_addr);
        }
    ).0
}

pub fn service() -> Scope {
    web::scope("/api")
        .service(
            web::scope("/v1")
                .service(ping::get_ping)
                .service(channels::get_channels)
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
