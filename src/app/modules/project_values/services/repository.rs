use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::project_values;
// use crate::database::schema::values;

use crate::app::modules::project_values::model::{ProjectValue, NewProjectValue};

pub async fn get_all(db: &Db) -> Result<Vec<ProjectValue>, diesel::result::Error> {
    let project_values = db.run(move |conn| {
        project_values::table
            .load::<ProjectValue>(conn)
    }).await;

    project_values
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<ProjectValue, diesel::result::Error> {
    let project_value = db.run(move |conn| {
        project_values::table
            .filter(project_values::id.eq(id))
            .first::<ProjectValue>(conn)
    }).await;

    project_value
}

pub async fn get_by_project_id(db: &Db, project_id: i32) -> Result<Vec<ProjectValue>, diesel::result::Error> {
    let project_values = db.run(move |conn| {
        project_values::table
            .filter(project_values::project_id.eq(project_id))
            .order(project_values::id.desc())
            .load::<ProjectValue>(conn)
    }).await;

    project_values
}

pub async fn create(db: &Db, new_project_value: NewProjectValue) -> Result<ProjectValue, diesel::result::Error> {
    let project_value = db.run(move |conn| {
        diesel::insert_into(project_values::table)
            .values(&new_project_value)
            .get_result::<ProjectValue>(conn)
    }).await;

    project_value
}

pub async fn update(db: &Db, id: i32, new_project_value: NewProjectValue) -> Result<ProjectValue, diesel::result::Error> {
    let project_value = db.run(move |conn| {
        diesel::update(project_values::table)
            .filter(project_values::id.eq(id))
            .set(new_project_value)
            .get_result::<ProjectValue>(conn)
    }).await;

    project_value
}
