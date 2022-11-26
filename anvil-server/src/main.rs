mod config;
mod notify;
mod api;
mod utils;
mod db;
mod log;

use std::{io};

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_ansi_support::enable_ansi_support().unwrap_or(());

    info!("<bright-black>===================================================</>");
    info!("<cyan>anvil: server </> <bright-black>Copyright (c) 2022 Finn Behrend</>");
    info!("<cyan>anvil: version</> <red>{}</> <cyan>on</> <red>{}</>", utils::build_version_string(), utils::build_system_info());
    println!();

    config::load_all();
    println!();

    notify::init();
    println!();

    db::init();
    println!();

    api::init().await
}
