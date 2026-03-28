// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Varchar,
        token -> Varchar,
        user_id -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        pin_hash -> Varchar,
        balance -> Numeric,
    }
}

diesel::joinable!(transactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(transactions, users,);
