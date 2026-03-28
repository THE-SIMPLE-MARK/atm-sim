use argon2::{
  Argon2, PasswordHasher,
  password_hash::{SaltString, rand_core::OsRng},
};
use diesel::prelude::*;

pub fn connect_db() -> PgConnection {
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
  PgConnection::establish(&database_url).unwrap_or_else(|e| panic!("Error connecting to DB: {}", e))
}

/// hashes & salts the given string via Argon2id
pub fn hash_string(str: String) -> String {
  let salt = SaltString::generate(OsRng);
  let argon2 = Argon2::default();

  return argon2
    .hash_password(str.as_bytes(), &salt)
    .unwrap()
    .to_string();
}
