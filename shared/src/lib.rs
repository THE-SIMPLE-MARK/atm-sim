use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PinVerifyRequest {
  pub account_id: String,
  pub pin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinVerifyResponse {
  pub is_valid: bool,
  pub transaction_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
  AccountNotFound,
  InvalidPasswordHash,
  InternalServerError,
}

impl ErrorCode {
  pub fn as_str(&self) -> &'static str {
    match self {
      ErrorCode::AccountNotFound => "ACCOUNT_NOT_FOUND",
      ErrorCode::InvalidPasswordHash => "INVALID_PASSWORD_HASH",
      ErrorCode::InternalServerError => "INTERNAL_SERVER_ERROR",
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
  pub code: ErrorCode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCardRequest {
  pub pin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCardResponse {
  pub success: bool,
}
