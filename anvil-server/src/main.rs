mod config;
mod notify;
mod api;
mod utils;

use std::{io};
use actix_web::{App, HttpServer, middleware};
use paris::{error, info, success};
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_ansi_support::enable_ansi_support().unwrap_or(());

    info!("<bright-black>==================================================</>");
    info!("<cyan>anvil: server</> <red>{}</> <cyan>on</> <red>{}</>", utils::build_version_string(), utils::build_system_info());
    println!();

    config::load();
    notify::init();

    // TODO: move this to a separate file
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config::get().database_config.username,
        config::get().database_config.password,
        config::get().database_config.address,
        config::get().database_config.port,
        config::get().database_config.database
    );

    // Create a connection to the database and if error BadConnection, log the message and exit
    let conn = PgConnection::establish(&db_url).unwrap_or_else(|e| {
        error!("Failed to connect to database!");
        error!("Error: {}", e);
        std::process::exit(1);
    });

    success!("connected to database");

    // todo!("Database migrations");

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

        .bind((config::get().bind_config.address.clone(), config::get().bind_config.port.clone()))?
        .run()
        .await
}
