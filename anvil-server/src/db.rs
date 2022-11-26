use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn init() {
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config::main::get().database_config.username,
        config::main::get().database_config.password,
        config::main::get().database_config.address,
        config::main::get().database_config.port,
        config::main::get().database_config.database
    );

    let mut conn = PgConnection::establish(&db_url).unwrap_or_else(|e| {
        error!("Failed to connect to database!");
        error!("Error: {}", e);
        std::process::exit(1);
    });
    cstm!("📊", "linked with database");

    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
    cstm!("📊", "ran migrations");
}
