use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::projects::model::{Project, NewProject};
use crate::app::modules::projects::services::repository as projects_repository;

pub async fn post_create_admin(db: &Db, _admin: UserInClaims, new_project: NewProject) -> Result<Json<Project>, Status> {
    let project = projects_repository::create(&db, new_project).await;

    match project {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}
