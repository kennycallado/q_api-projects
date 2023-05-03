use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::projects::model::Project;
use crate::app::modules::projects::services::repository as projects_repository;

pub async fn get_show_admin(db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<Project>, Status> {
    let project = projects_repository::get_by_id(&db, id).await;

    match project {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}
