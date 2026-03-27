use diesel::prelude::*;

pub fn connect_db() -> PgConnection {
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
  PgConnection::establish(&database_url).unwrap_or_else(|e| panic!("Error connecting to DB: {}", e))
}
