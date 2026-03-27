diesel::table! {
  users {
    id -> VarChar,
    pinHash -> VarChar,
    name -> VarChar,
    balance -> Decimal,
  }
}
