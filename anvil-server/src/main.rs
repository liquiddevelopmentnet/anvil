mod config;
mod notify;

use std::{fs, io};
use actix_web::{App, HttpServer, middleware};
use actix_web::cookie::time::format_description::well_known::iso8601::Config;
use actix_web::web::PayloadConfig;
use paris::{error, info, success, warn};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::config::Configuration;

pub static mut CONFIG: Option<Configuration> = None;

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_ansi_support::enable_ansi_support().unwrap_or(());

    #[cfg(debug_assertions)]
        let build_type = "development";
    #[cfg(not(debug_assertions))]
        let build_type = "production";

    info!("<bright-black>=======================================</>");
    info!("<blue>anvil: server {}</>", env!("CARGO_PKG_VERSION"));
    info!("");
    info!("<bright-black>build     :</> {}: {}<bright-black>@</>{}", build_type, &env!("VERGEN_GIT_SHA")[..7], &env!("VERGEN_GIT_BRANCH"));
    info!("<bright-black>system    :</> {}<bright-black>/</>{}", std::env::consts::OS, std::env::consts::ARCH);
    info!("");

    unsafe {
        notify::init();

        let lcc = config::load();
        CONFIG = Option::from(lcc.clone());

        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            lcc.database_config.username,
            lcc.database_config.password,
            lcc.database_config.address,
            lcc.database_config.port,
            lcc.database_config.database
        );

        let conn = PgConnection::establish(&db_url).expect("Failed to connect to database");

        info!("connected to database");
    }

    /*todo!("load plugins");

    todo!("load routes");

    todo!("load middleware");

    todo!("load server");*/

    Ok(())
}
