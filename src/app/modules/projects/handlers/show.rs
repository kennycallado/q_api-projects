use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::Db;

use crate::app::providers::interfaces::helpers::claims::UserInClaims;

use crate::app::modules::records::model::Record;
use crate::app::modules::projects::model::{ProjectWithRecords, Project};

use crate::app::modules::project_records::services::repository as pr_repository;
use crate::app::modules::records::services::repository as records_repository;
use crate::app::modules::projects::services::repository as projects_repository;

pub async fn get_show_admin(db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<Project>, Status> {
    let project = projects_repository::get_by_id(&db, id).await;

    match project {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn get_show_records_admin(db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<ProjectWithRecords>, Status> {
    let project = projects_repository::get_by_id(&db, id).await;

    match project {
        Ok(project) => { 
            let pr = pr_repository::get_by_project_id(&db, id).await;

            let records: Option<Vec<Record>> = match pr {
                Ok(project_records) => {
                    let ids = project_records.iter().map(|pr| pr.records_id).collect::<Vec<i32>>();
                    let record = records_repository::get_by_multiple_ids(&db, ids).await;

                    match record {
                        Ok(records) => Some(records),
                        Err(_) => return Err(Status::InternalServerError),
                    }
                },
                Err(_) => None,
            };

            let project = ProjectWithRecords {
                id: project.id,
                name: project.name,
                keys: project.keys,
                records,
            };

            Ok(Json(project))
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

// pub async fn get_show_values_admin(db: &Db, _admin: UserInClaims, id: i32) -> Result<Json<ProjectWithValues>, Status> {
//     let project = projects_repository::get_by_id(&db, id).await;
//     if let Err(_) = project {
//         return Err(Status::InternalServerError);
//     }
//     let project = project.unwrap();
//     
//     let project_values = pv_repository::get_by_project_id(&db, id).await;
//     if let Err(_) = project_values {
//         return Err(Status::InternalServerError);
//     }
//     let project_values = project_values.unwrap();

//     let mut values = vec![];
//     for pv in project_values {
//         let value = values_repository::get_by_id(&db, pv.values_id).await;
//         if let Err(_) = value {
//             return Err(Status::InternalServerError);
//         }
//         let value = value.unwrap();

//         values.push(value);
//     }

//     let project = ProjectWithValues {
//         id: project.id,
//         name: project.name,
//         keys: project.keys,
//         values: Some(values),
//     };

//     Ok(Json(project))
// }

// pub async fn get_show_user_values_admin(db: &Db, _admin: UserInClaims, id: i32, user_id: i32) -> Result<Json<ProjectWithValuesAndUser>, Status> {
//     // get the project
//     let project = projects_repository::get_by_id(&db, id).await;
//     if let Err(_) = project {
//         return Err(Status::InternalServerError);
//     }
//     let project = project.unwrap();

//     // get the pv by user_id (relation between project and values)
//     let project_values = pv_repository::get_by_user_id(&db, user_id).await;
//     if let Err(_) = project_values {
//         return Err(Status::InternalServerError);
//     }
//     let project_values = project_values.unwrap();

//     // collect the values ids
//     let values_ids: Vec<i32> = project_values.iter().map(|pv| pv.values_id).collect();

//     // get the values
//     let values = values_repository::get_by_ids(&db, values_ids).await;
//     if let Err(_) = values {
//         return Err(Status::InternalServerError);
//     }
//     let values = values.unwrap();
//     
//     let project = ProjectWithValuesAndUser {
//         id: project.id,
//         user_id,
//         name: project.name,
//         keys: project.keys,
//         values: Some(values),
//     };

//     Ok(Json(project))
// }
