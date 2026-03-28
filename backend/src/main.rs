use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
  Json, Router,
  http::StatusCode,
  response::Result,
  routing::{get, post},
};
use backend::{connect_db, hash_string};
use cuid2::cuid;
use diesel::{dsl::insert_into, prelude::*};
use shared::{ErrorCode, ErrorResponse, PinVerifyRequest, PinVerifyResponse};

use crate::schema::{transactions, users};

mod schema;

#[tokio::main]
async fn main() {
  #[cfg(debug_assertions)]
  dotenvy::dotenv().ok();

  let app = Router::new()
    .route("/ping", get(ping))
    .route("/verify-pin", post(verify_pin));

  let addr = std::env::var("BACKEND_ADDRESS").expect("BACKEND_ADDRESS must be set.");
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

  println!("Listening on {}", &addr);
  axum::serve(listener, app).await.unwrap();
}

async fn ping() -> (StatusCode, String) {
  (StatusCode::OK, "Pong".to_string())
}

async fn verify_pin(
  Json(data): Json<PinVerifyRequest>,
) -> Result<Json<PinVerifyResponse>, (StatusCode, Json<ErrorResponse>)> {
  let mut conn = connect_db();

  let user_pin_hash = users::table
    .find(&data.account_id)
    .select(users::pin_hash)
    .first::<String>(&mut conn)
    .map_err(|_| {
      (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
          code: ErrorCode::AccountNotFound,
        }),
      )
    })?;

  let is_valid = Argon2::default()
    .verify_password(
      data.pin.as_bytes(),
      &PasswordHash::new(&user_pin_hash).map_err(|_| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(ErrorResponse {
            code: ErrorCode::InvalidPasswordHash,
          }),
        )
      })?,
    )
    .is_ok();

  let transaction_token = if is_valid {
    let transaction_id = cuid().to_string();
    let token = cuid().to_string();
    let token_hash = hash_string(token.clone());

    let _ = insert_into(transactions::table)
      .values((
        transactions::id.eq(&transaction_id),
        transactions::token.eq(&token_hash),
        transactions::user_id.eq(&data.account_id),
      ))
      .execute(&mut conn);

    Some(token)
  } else {
    None
  };

  Ok(Json(PinVerifyResponse {
    is_valid,
    transaction_token,
  }))
}
