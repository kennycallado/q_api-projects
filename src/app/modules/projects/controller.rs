use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::guards::claims::AccessClaims;
use crate::config::database::Db;

use crate::app::modules::projects::handlers::{create, index, show, update};
use crate::app::modules::projects::model::{Project, NewProject};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_index,
        options_show,
        get_index,
        get_index_none,
        get_show,
        get_show_none,
        post_create,
        post_create_none,
        put_update,
        put_update_none,
    ]
}

#[options("/")]
pub async fn options_index() -> Status {
    Status::Ok
}

#[options("/<_id>")]
pub async fn options_show(_id: i32) -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: Db, claims: AccessClaims) -> Result<Json<Vec<Project>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_admin(&db, claims.0.user).await,
        _       => {
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

#[get("/<id>", rank = 1)]
pub async fn get_show(db: Db, claims: AccessClaims, id: i32) -> Result<Json<Project>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(&db, claims.0.user, id).await ,
        _       => {
            println!(
                "Error: get_index; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_id>", rank = 2)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_project>",rank = 1)]
pub async fn post_create(db: Db, claims: AccessClaims, new_project: Json<NewProject>) -> Result<Json<Project>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => create::post_create_admin(&db, claims.0.user, new_project.into_inner()).await,
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

#[post("/", data = "<_new_project>",rank = 2)]
pub async fn post_create_none(_new_project: Json<NewProject>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<new_project>", rank = 1)]
pub async fn put_update(db: Db, claims: AccessClaims, id: i32, new_project: Json<NewProject>) -> Result<Json<Project>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::put_update_admin(&db, claims.0.user, id, new_project.into_inner()).await,
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
    // update::put_update_admin(&db, id, new_project.into_inner()).await
}

#[put("/<_id>", data = "<_new_project>", rank = 2)]
pub async fn put_update_none(_id: i32, _new_project: Json<NewProject>) -> Status {
    Status::Unauthorized
}
