use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::modules::records::model::{NewRecord, Record};
use crate::app::providers::guards::claims::AccessClaims;

use crate::app::modules::projects::handlers::{create, index, show, update};
use crate::app::modules::projects::model::{NewProject, Project, ProjectWithRecords};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_all,
        get_index,
        get_index_none,
        get_show,
        get_show_none,
        get_show_record,
        get_show_record_none,
        get_show_records_last,
        get_show_records_last_none,
        get_show_user,
        get_show_user_none,
        get_show_user_new,
        get_show_user_new_none,
        post_create,
        post_create_none,
        post_create_record,
        post_create_record_none,
        put_update,
        put_update_none,
    ]
}

#[options("/<_..>")]
pub fn options_all() -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: &Db, claims: AccessClaims) -> Result<Json<Vec<Project>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_admin(db, claims.0.user).await,
        _ => {
            println!(
                "Error: get_index; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/", rank = 2)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 101)]
pub async fn get_show(db: &Db, claims: AccessClaims, id: i32) -> Result<Json<Project>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(db, claims.0.user, id).await,
        _ => {
            println!(
                "Error: get_index; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_id>", rank = 102)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[get("/<id>/record", rank = 1)]
pub async fn get_show_record(
    db: &Db,
    claims: AccessClaims,
    id: i32,
) -> Result<Json<ProjectWithRecords>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => show::get_show_records_admin(db, claims.0.user, id).await,
        _ => {
            println!(
                "Error: get_show_records; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_id>/record", rank = 2)]
pub async fn get_show_record_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[get("/<id>/record/lasts", rank = 1)]
pub async fn get_show_records_last(
    db: &Db,
    claims: AccessClaims,
    id: i32,
) -> Result<Json<ProjectWithRecords>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => show::get_show_last_records_admin(db, claims.0.user, id).await,
        _ => {
            println!(
                "Error: get_show_records_last; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_id>/record/lasts", rank = 2)]
pub async fn get_show_records_last_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[get("/<id>/user/<user_id>", rank = 101)]
pub async fn get_show_user(
    db: &Db,
    claims: AccessClaims,
    id: i32,
    user_id: i32,
) -> Result<Json<Vec<Record>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_user_admin(db, claims.0.user, id, user_id).await,
        _ => {
            println!(
                "Error: get_show_user; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_id>/user/<_user_id>", rank = 102)]
pub async fn get_show_user_none(_id: i32, _user_id: i32) -> Status {
    Status::Unauthorized
}

#[get("/<project_id>/user/<user_id>/new", rank = 1)]
pub async fn get_show_user_new(
    db: &Db,
    claims: AccessClaims,
    project_id: i32,
    user_id: i32,
) -> Result<Json<Project>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => {
            create::get_show_user_new_admin(db, claims.0.user, project_id, user_id).await
        }
        _ => {
            println!(
                "Error: get_show_user_new; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_project_id>/user/<_user_id>/new", rank = 2)]
pub async fn get_show_user_new_none(_project_id: i32, _user_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_project>", rank = 1)]
pub async fn post_create(
    db: &Db,
    claims: AccessClaims,
    new_project: Json<NewProject>,
) -> Result<Json<Project>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => {
            create::post_create_admin(db, claims.0.user, new_project.into_inner()).await
        }
        // "coord" => create::post_create_coord(db, claims.0.user, new_project.into_inner()).await,
        // "thera" => create::post_create_thera(db, claims.0.user, new_project.into_inner()).await,
        _ => {
            println!(
                "Error: post_create; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
    // create::post_create_admin(&db, new_project.into_inner()).await
}

#[post("/", data = "<_new_project>", rank = 2)]
pub async fn post_create_none(_new_project: Json<NewProject>) -> Status {
    Status::Unauthorized
}

#[post("/<id>/record", data = "<new_record>", rank = 1)]
pub async fn post_create_record(
    db: &Db,
    claims: AccessClaims,
    id: i32,
    new_record: Json<NewRecord>,
) -> Result<Json<Record>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => {
            create::post_add_record_admin(db, claims.0.user, id, new_record.into_inner()).await
        }
        _ => {
            println!(
                "Error: post_create_record; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[post("/<_id>/record", data = "<_new_record>", rank = 2)]
pub async fn post_create_record_none(_id: i32, _new_record: Json<NewRecord>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<new_project>", rank = 1)]
pub async fn put_update(
    db: &Db,
    claims: AccessClaims,
    id: i32,
    new_project: Json<NewProject>,
) -> Result<Json<Project>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => {
            update::put_update_admin(db, claims.0.user, id, new_project.into_inner()).await
        }
        // "coord" => update::put_update_coord(&db, claims.0.user, id, new_project.into_inner()).await,
        // "thera" => update::put_update_thera(&db, claims.0.user, id, new_project.into_inner()).await,
        _ => {
            println!(
                "Error: post_create; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[put("/<_id>", data = "<_new_project>", rank = 2)]
pub async fn put_update_none(_id: i32, _new_project: Json<NewProject>) -> Status {
    Status::Unauthorized
}
