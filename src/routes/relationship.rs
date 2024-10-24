use crate::auth::user::AuthenticatedUser;
use crate::db;
use crate::models::relationship::{
    DevPmRelationship, InvitePmRequest, NewDevPmRelationship, RespondToInviteRequest,
};
use crate::models::snack::Snack;
use crate::models::user::User;
use crate::schema::dev_pm_relationships::{self, developer_id, project_manager_id, status};
use crate::schema::snacks::{self, user_id};
use crate::schema::users;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

#[post("/invite-pm", data = "<invite_data>")]
pub fn invite_project_manager(
    invite_data: Json<InvitePmRequest>,
    user: AuthenticatedUser,
) -> Result<Json<DevPmRelationship>, Status> {
    let mut conn = db::establish_connection();

    if user.0.role != "developer" {
        return Err(Status::Forbidden);
    }

    let pm = users::table
        .find(invite_data.project_manager_id)
        .first::<User>(&mut conn)
        .map_err(|_| Status::NotFound)?;

    if pm.role != "project_manager" {
        return Err(Status::BadRequest);
    }

    let existing = DevPmRelationship::for_developer(user.0.id)
        .filter(project_manager_id.eq(invite_data.project_manager_id))
        .filter(status.ne("rejected"))
        .first::<DevPmRelationship>(&mut conn);

    if existing.is_ok() {
        return Err(Status::BadRequest);
    }

    let new_relationship = NewDevPmRelationship {
        developer_id: user.0.id,
        project_manager_id: invite_data.project_manager_id,
        status: "pending".to_string(),
    };

    diesel::insert_into(dev_pm_relationships::table)
        .values(&new_relationship)
        .get_result(&mut conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/my-developers")]
pub fn list_developers(user: AuthenticatedUser) -> Result<Json<Vec<User>>, Status> {
    let mut conn = db::establish_connection();

    if user.0.role != "project_manager" {
        return Err(Status::Forbidden);
    }

    users::table
        .inner_join(
            dev_pm_relationships::table.on(developer_id
                .eq(users::id)
                .and(project_manager_id.eq(user.0.id))
                .and(status.eq("accepted"))),
        )
        .select(users::all_columns)
        .load::<User>(&mut conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/snacks")]
pub fn list_snacks(user: AuthenticatedUser) -> Result<Json<Vec<Snack>>, Status> {
    let mut conn = db::establish_connection();

    match user.0.role.as_str() {
        "admin" => snacks::table.limit(100).select(Snack::as_select()).load(&mut conn),
        "project_manager" => snacks::table
            .inner_join(
                dev_pm_relationships::table.on(user_id
                    .eq(developer_id)
                    .and(project_manager_id.eq(user.0.id))
                    .and(status.eq("accepted"))),
            )
            .select(Snack::as_select())
            .distinct()
            .limit(100)
            .load(&mut conn),
        _ => snacks::table
            .filter(user_id.eq(user.0.id))
            .limit(100)
            .select(Snack::as_select())
            .load(&mut conn),
    }
    .map(Json)
    .map_err(|err| {
        println!("Database error: {:?}", err);
        Status::InternalServerError
    })
}

#[patch("/respond-to-invite/<relationship_id>", data = "<response_data>")]
pub fn respond_to_invite(
    relationship_id: i32,
    response_data: Json<RespondToInviteRequest>,
    user: AuthenticatedUser,
) -> Result<Json<DevPmRelationship>, Status> {
    let mut conn = db::establish_connection();

    if user.0.role != "project_manager" {
        return Err(Status::Forbidden);
    }

    let relationship = DevPmRelationship::for_project_manager(user.0.id)
        .filter(dev_pm_relationships::id.eq(relationship_id))
        .first::<DevPmRelationship>(&mut conn)
        .map_err(|_| Status::NotFound)?;

    if relationship.status != "pending" {
        return Err(Status::BadRequest);
    }

    diesel::update(dev_pm_relationships::table.find(relationship_id))
        .set(status.eq(&response_data.status))
        .get_result(&mut conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
