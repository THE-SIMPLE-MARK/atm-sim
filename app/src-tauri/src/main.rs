// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schema;
use atm_sim_lib::connect_db;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn main() {
  let mut connection = connect_db();
  if let Err(e) = connection.run_pending_migrations(MIGRATIONS) {
    panic!("Error running pending migrations: {}", e);
  }

  atm_sim_lib::run()
}
