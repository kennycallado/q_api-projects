use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::modules::records::model::{NewRecord, Record};
use crate::app::modules::project_records::model::NewProjectRecord;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::projects::model::{Project, NewProject};
use crate::app::modules::projects::services::repository as projects_repository;

use crate::app::modules::records::services::repository as records_repository;
use crate::app::modules::project_records::services::repository as pr_repository;

pub async fn post_create_admin(db: &Db, _admin: UserInClaims, new_project: NewProject) -> Result<Json<Project>, Status> {
    let project = projects_repository::create(&db, new_project).await;

    match project {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn post_add_record_admin(db: &Db, _admin: UserInClaims, project_id: i32, new_record: NewRecord) -> Result<Json<Record>, Status> {
    let record = records_repository::create(&db, new_record).await;

    match record {
        Ok(record) => {
            let new_project_record = NewProjectRecord {
                project_id,
                record_id: record.id,
            };

            let project_record = pr_repository::create(&db, new_project_record).await;

            match project_record {
                Ok(_) => Ok(Json(record)),
                Err(_) => Err(Status::InternalServerError),
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
