use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::projects::model::{Project, NewProject};
use crate::app::modules::projects::services::repository as projects_repository;

pub async fn put_update_admin(db: &Db, _admin: UserInClaims, id: i32, new_project: NewProject) -> Result<Json<Project>, Status> {
    let project = projects_repository::update(&db, id, new_project).await;

    match project {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}
