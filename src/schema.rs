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
    }
}
