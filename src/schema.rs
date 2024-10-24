// @generated automatically by Diesel CLI.

diesel::table! {
    snacks (id) {
        id -> Int4,
        name -> Varchar,
        category -> Varchar,
        price -> Numeric,
        image_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(snacks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    snacks,
    users,
);
