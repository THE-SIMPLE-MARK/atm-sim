use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
pub fn connect_db() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
  PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to DB!"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

  #[cfg(not(feature = "debug"))]
  let builder = builder.plugin(tauri_plugin_prevent_default::init());

  builder
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
