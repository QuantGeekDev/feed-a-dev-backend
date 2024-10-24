use crate::models::user::User;
use crate::schema::dev_pm_relationships;
use chrono::NaiveDateTime;
use diesel::pg::Pg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Identifiable, Associations, Debug)]
#[diesel(belongs_to(User, foreign_key = developer_id))]
#[diesel(table_name = crate::schema::dev_pm_relationships)]
pub struct DevPmRelationship {
    pub id: i32,
    pub developer_id: i32,
    pub project_manager_id: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::dev_pm_relationships)]
pub struct NewDevPmRelationship {
    pub developer_id: i32,
    pub project_manager_id: i32,
    pub status: String,
}

#[derive(Deserialize)]
pub struct InvitePmRequest {
    pub project_manager_id: i32,
}

#[derive(Deserialize)]
pub struct RespondToInviteRequest {
    pub status: String,
}

impl DevPmRelationship {
    pub fn for_developer(user_id: i32) -> dev_pm_relationships::BoxedQuery<'static, Pg> {
        use crate::schema::dev_pm_relationships::dsl::*;

        dev_pm_relationships.filter(developer_id.eq(user_id)).into_boxed()
    }

    pub fn for_project_manager(user_id: i32) -> dev_pm_relationships::BoxedQuery<'static, Pg> {
        use crate::schema::dev_pm_relationships::dsl::*;

        dev_pm_relationships.filter(project_manager_id.eq(user_id)).into_boxed()
    }

    pub fn pending() -> dev_pm_relationships::BoxedQuery<'static, Pg> {
        use crate::schema::dev_pm_relationships::dsl::*;

        dev_pm_relationships.filter(status.eq("pending")).into_boxed()
    }

    pub fn accepted() -> dev_pm_relationships::BoxedQuery<'static, Pg> {
        use crate::schema::dev_pm_relationships::dsl::*;

        dev_pm_relationships.filter(status.eq("accepted")).into_boxed()
    }
}
