use diesel::prelude::*;

use crate::database::connection::Db;
use crate::database::schema::projects;

use crate::app::modules::projects::model::{NewProject, Project};

pub async fn get_all(db: &Db) -> Result<Vec<Project>, diesel::result::Error> {
    let projects = db
        .run(move |conn| projects::table.load::<(i32, String, Vec<Option<String>>)>(conn))
        .await?;

    let projects = projects
        .into_iter()
        .map(|(id, name, keys)| Project {
            id,
            name,
            keys: Some(keys
                .into_iter()
                .filter_map(|key| key)
                .collect::<Vec<String>>()),
        })
        .collect::<Vec<Project>>();

    Ok(projects)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Project, diesel::result::Error> {
    let project = db
        .run(move |conn| {
            projects::table
                .filter(projects::id.eq(id))
                .first::<(i32, String, Vec<Option<String>>)>(conn)
        })
        .await?;

    let project = Project {
        id: project.0,
        name: project.1,
        keys: Some(project.2.into_iter().filter_map(|key| key).collect::<Vec<String>>()),
    };

    Ok(project)
}

pub async fn create(
    db: &Db,
    new_project: NewProject,
) -> Result<Project, diesel::result::Error> {
    let project = db
        .run(move |conn| {
            diesel::insert_into(projects::table)
                .values(&new_project)
                .get_result::<(i32, String, Vec<Option<String>>)>(conn)
        })
        .await?;

    let project = Project {
        id: project.0,
        name: project.1,
        keys: Some(project.2.into_iter().filter_map(|key| key).collect::<Vec<String>>()),
    };

    Ok(project)
}

pub async fn update(
    db: &Db,
    id: i32,
    new_project: NewProject,
) -> Result<Project, diesel::result::Error> {
    let project = db
        .run(move |conn| {
            diesel::update(projects::table)
                .filter(projects::id.eq(id))
                .set(&new_project)
                .get_result::<(i32, String, Vec<Option<String>>)>(conn)
        })
        .await?;

    let project = Project {
        id: project.0,
        name: project.1,
        keys: Some(project.2.into_iter().filter_map(|key| key).collect::<Vec<String>>()),
    };

    Ok(project)
}
