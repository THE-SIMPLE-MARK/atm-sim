#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

  #[cfg(not(feature = "debug"))]
  let builder = builder.plugin(tauri_plugin_prevent_default::init());

  builder
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
