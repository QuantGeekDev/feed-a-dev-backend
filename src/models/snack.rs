use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::snacks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Snack {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub price: rust_decimal::Decimal,
    pub image_url: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct NewSnackData {
    pub name: String,
    pub category: String,
    pub price: f64,
    pub image_url: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::snacks)]
pub struct NewSnack {
    pub name: String,
    pub category: String,
    pub price: rust_decimal::Decimal,
    pub image_url: String,
}

impl From<NewSnackData> for NewSnack {
    fn from(data: NewSnackData) -> Self {
        NewSnack {
            name: data.name,
            category: data.category,
            price: rust_decimal::Decimal::from_f64(data.price).unwrap(),
            image_url: data.image_url,
        }
    }
}