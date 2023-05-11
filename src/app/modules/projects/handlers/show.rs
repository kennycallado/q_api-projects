use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::project_values::services::repository as pv_repository;
use crate::app::modules::values::services::repository as values_repository;

use crate::app::modules::projects::model::{Project, ProjectWithValues};
use crate::app::modules::projects::services::repository as projects_repository;

pub async fn get_show_admin(db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<Project>, Status> {
    let project = projects_repository::get_by_id(&db, id).await;

    match project {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn get_show_values_admin(db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<ProjectWithValues>, Status> {
    let project = projects_repository::get_by_id(&db, id).await;
    if let Err(_) = project {
        return Err(Status::InternalServerError);
    }
    let project = project.unwrap();
    
    let project_values = pv_repository::get_by_project_id(&db, id).await;
    if let Err(_) = project_values {
        return Err(Status::InternalServerError);
    }
    let project_values = project_values.unwrap();

    let mut values = vec![];
    for pv in project_values {
        let value = values_repository::get_by_id(&db, pv.values_id).await;
        if let Err(_) = value {
            return Err(Status::InternalServerError);
        }
        let value = value.unwrap();

        values.push(value);
    }

    let project = ProjectWithValues {
        id: project.id,
        name: project.name,
        keys: project.keys,
        values: Some(values),
    };

    Ok(Json(project))
}

pub async fn get_show_user_values_admin(db: &Db, _admin: UserInClaims, id: i32, user_id: i32) -> Result<Json<ProjectWithValues>, Status> {
    let project = projects_repository::get_by_id(&db, id).await;
    if let Err(_) = project {
        return Err(Status::InternalServerError);
    }
    let project = project.unwrap();
    
    let values = values_repository::get_all_by_user_id(&db, user_id).await;
    if let Err(_) = values {
        return Err(Status::InternalServerError);
    }
    let values = values.unwrap();

    let project = ProjectWithValues {
        id: project.id,
        name: project.name,
        keys: project.keys,
        values: Some(values),
    };

    Ok(Json(project))
}
