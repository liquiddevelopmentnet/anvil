use diesel::{Connection, PgConnection};
use crate::config;
use crate::log;
use crate::{error, success};

pub fn init() {
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config::main::get().database_config.username,
        config::main::get().database_config.password,
        config::main::get().database_config.address,
        config::main::get().database_config.port,
        config::main::get().database_config.database
    );

    let conn = PgConnection::establish(&db_url).unwrap_or_else(|e| {
        error!("Failed to connect to database!");
        error!("Error: {}", e);
        std::process::exit(1);
    });

    success!("connected to database");

    // todo!("Database migrations");
}
