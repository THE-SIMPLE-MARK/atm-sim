use serde::Serialize;
use shared::{ErrorCode, ErrorResponse, PinVerifyRequest, PinVerifyResponse};

const BACKEND_URL: &str = env!("BACKEND_URL");

#[derive(Serialize)]
pub struct CommandErrorResponse {
  pub code: ErrorCode,
}

fn internal_error<E: std::error::Error>(e: E) -> CommandErrorResponse {
  eprintln!("Internal error: {}", e);
  CommandErrorResponse {
    code: ErrorCode::InternalServerError,
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());

  #[cfg(not(feature = "debug"))]
  let builder = builder.plugin(tauri_plugin_prevent_default::init());

  builder
    .invoke_handler(tauri::generate_handler![verify_pin])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn verify_pin(
  account_id: String,
  pin: String,
) -> Result<PinVerifyResponse, CommandErrorResponse> {
  let client = reqwest::Client::new();

  let response = client
    .post(&format!("{}/verify-pin", BACKEND_URL))
    .json(&PinVerifyRequest { account_id, pin })
    .send()
    .await
    .map_err(internal_error)?;

  if !response.status().is_success() {
    let error_response = response
      .json::<ErrorResponse>()
      .await
      .map_err(internal_error)?;

    return Err(CommandErrorResponse {
      code: error_response.code,
    });
  }

  let result = response
    .json::<PinVerifyResponse>()
    .await
    .map_err(internal_error)?;

  Ok(result)
}
