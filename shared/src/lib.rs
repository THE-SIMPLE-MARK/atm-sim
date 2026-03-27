use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PinVerifyRequest {
  pub pin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinVerifyResponse {
  pub is_valid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCardRequest {
  pub pin: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCardResponse {
  pub success: bool,
}
