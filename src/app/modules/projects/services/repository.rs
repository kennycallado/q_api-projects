use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::projects;

use crate::app::modules::projects::model::{Project, NewProject};

pub async fn get_all(db: &Db) -> Result<Vec<Project>, diesel::result::Error> {
    let projects = db.run(move |conn| {
        projects::table
            .load::<Project>(conn)
    }).await?;

    Ok(projects)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Project, diesel::result::Error> {
    let project = db.run(move |conn| {
        projects::table
            .filter(projects::id.eq(id))
            .first::<Project>(conn)
    }).await?;

    Ok(project)
}

pub async fn create(db: &Db, new_project: NewProject) -> Result<Project, diesel::result::Error> {
    let project = db.run(move |conn| {
        diesel::insert_into(projects::table)
            .values(&new_project)
            .get_result::<Project>(conn)
    }).await?;

    Ok(project)
}

pub async fn update(db: &Db, id: i32, new_project: NewProject) -> Result<Project, diesel::result::Error> {
    let project = db.run(move |conn| {
        diesel::update(projects::table)
            .filter(projects::id.eq(id))
            .set(&new_project)
            .get_result::<Project>(conn)
    }).await?;

    Ok(project)
}
