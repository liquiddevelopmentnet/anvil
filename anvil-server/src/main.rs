use std::{fs, io};
use actix_web::{App, HttpServer, middleware};
use actix_web::cookie::time::format_description::well_known::iso8601::Config;
use actix_web::web::PayloadConfig;
use paris::{error, info, success, warn};

#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    todo!("load config");

    todo!("load database");

    todo!("load plugins");

    todo!("load routes");

    todo!("load middleware");

    todo!("load server");

    Ok(())
}
