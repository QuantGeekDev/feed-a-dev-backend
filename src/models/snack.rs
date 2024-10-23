use chrono::NaiveDateTime;
use diesel::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, AsChangeset, Associations)]
#[diesel(table_name = crate::schema::snacks)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Snack {
    pub id: i32,
    pub name: String,
    pub category: String,
    #[diesel(sql_type = Numeric)]
    pub price: Decimal,
    pub image_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::snacks)]
pub struct NewSnack {
    pub name: String,
    pub category: String,
    #[diesel(sql_type = Numeric)]
    pub price: Decimal,
    pub image_url: String,
}

