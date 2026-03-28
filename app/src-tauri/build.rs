fn main() {
  dotenvy::from_filename("../../.env.tauri").ok();

  // pass all env vars from build stage to app
  for (key, value) in std::env::vars() {
    println!("cargo:rustc-env={}={}", key, value);
  }

  tauri_build::build()
}
