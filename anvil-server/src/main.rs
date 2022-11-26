mod config;
mod notify;
mod api;
mod utils;
mod db;

use std::{io};
use actix_web::{HttpServer, middleware};
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
    api::init().await
}
