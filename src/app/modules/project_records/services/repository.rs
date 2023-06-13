use diesel::prelude::*;

use crate::database::connection::Db;
use crate::database::schema::project_records;

use crate::app::modules::project_records::model::{NewProjectRecord, ProjectRecord};

pub async fn get_all(db: &Db) -> Result<Vec<ProjectRecord>, diesel::result::Error> {
    let project_records = db
        .run(move |conn| project_records::table.load::<ProjectRecord>(conn))
        .await;

    project_records
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<ProjectRecord, diesel::result::Error> {
    let project_records = db
        .run(move |conn| {
            project_records::table
                .filter(project_records::id.eq(id))
                .first::<ProjectRecord>(conn)
        })
        .await;

    project_records
}

pub async fn get_by_project_id(
    db: &Db,
    project_id: i32,
) -> Result<Vec<ProjectRecord>, diesel::result::Error> {
    let project_records = db
        .run(move |conn| {
            project_records::table
                .filter(project_records::project_id.eq(project_id))
                .order(project_records::id.desc())
                .load::<ProjectRecord>(conn)
        })
        .await;

    project_records
}

pub async fn create(
    db: &Db,
    new_project_records: NewProjectRecord,
) -> Result<ProjectRecord, diesel::result::Error> {
    let project_records = db
        .run(move |conn| {
            diesel::insert_into(project_records::table)
                .values(&new_project_records)
                .get_result::<ProjectRecord>(conn)
        })
        .await;

    project_records
}

pub async fn update(
    db: &Db,
    id: i32,
    new_project_records: NewProjectRecord,
) -> Result<ProjectRecord, diesel::result::Error> {
    let project_records = db
        .run(move |conn| {
            diesel::update(project_records::table)
                .filter(project_records::id.eq(id))
                .set(new_project_records)
                .get_result::<ProjectRecord>(conn)
        })
        .await;

    project_records
}
