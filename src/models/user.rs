use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use serde::Serialize;
#[derive(Queryable, Serialize, Identifiable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub role: String,
}