use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::projects::model::Project;
use crate::app::modules::projects::services::repository as projects_repository;

pub async fn get_index_admin(db: &Db, _admin: UserInClaims) -> Result<Json<Vec<Project>>, Status> {
    let projects = projects_repository::get_all(&db).await;

    match projects {
        Ok(projects) => Ok(Json(projects)),
        Err(_) => Err(Status::InternalServerError),
    }
}
