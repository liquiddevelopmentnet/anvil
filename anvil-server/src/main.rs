mod config;
mod notify;
mod api;
mod utils;
mod db;

use std::{io};
use actix_web::{App, HttpServer, middleware};
use paris::{info};

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_ansi_support::enable_ansi_support().unwrap_or(());

    info!("<bright-black>==================================================</>");
    info!("<cyan>anvil: server</> <red>{}</> <cyan>on</> <red>{}</>", utils::build_version_string(), utils::build_system_info());
    println!();

    config::load_all();

    notify::init();
    db::init();

    // todo!("Load plugins");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::DefaultHeaders::new()
                .add(("X-Powered-By", "anvil"))
            )
            .wrap(actix_cors::Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
            )
            .service(api::service())
    })

        .bind((config::main::get().bind_config.address.clone(), config::main::get().bind_config.port.clone()))?
        .run()
        .await
}
