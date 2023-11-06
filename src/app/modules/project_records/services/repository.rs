#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::database::connection::Db;

use crate::app::modules::project_records::model::{NewProjectRecord, ProjectRecord};

pub async fn get_all(db: &Db) -> Result<Vec<ProjectRecord>, sqlx::Error> {
    let project_records = sqlx::query_as!(ProjectRecord,
        "SELECT * FROM project_records")
        .fetch_all(&db.0)
        .await?;

    // let project_records = db
    //     .run(move |conn| project_records::table.load::<ProjectRecord>(conn))
    //     .await;

    Ok(project_records)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<ProjectRecord, sqlx::Error> {
    let project_record = sqlx::query_as!(ProjectRecord,
        "SELECT * FROM project_records WHERE id = $1", id)
        .fetch_one(&db.0)
        .await?;
    // let project_records = db
    //     .run(move |conn| {
    //         project_records::table
    //             .filter(project_records::id.eq(id))
    //             .first::<ProjectRecord>(conn)
    //     })
    //     .await;

    Ok(project_record)
}

pub async fn get_by_project_id(
    db: &Db,
    project_id: i32,
) -> Result<Vec<ProjectRecord>, sqlx::Error> {
    let project_records = sqlx::query_as!(ProjectRecord,
        "SELECT * FROM project_records WHERE project_id = $1 ORDER BY project_id DESC", project_id)
        .fetch_all(&db.0)
        .await?;

    // let project_records = db
    //     .run(move |conn| {
    //         project_records::table
    //             .filter(project_records::project_id.eq(project_id))
    //             .order(project_records::id.desc())
    //             .load::<ProjectRecord>(conn)
    //     })
    //     .await;

    Ok(project_records)
}

pub async fn create(
    db: &Db,
    new_project_records: NewProjectRecord,
) -> Result<ProjectRecord, sqlx::Error> {
    let project_record = sqlx::query_as!(ProjectRecord,
        "INSERT INTO project_records (project_id, record_id) VALUES ($1, $2) RETURNING *",
        new_project_records.project_id, new_project_records.record_id)
        .fetch_one(&db.0)
        .await?;

    // let project_records = db
    //     .run(move |conn| {
    //         diesel::insert_into(project_records::table)
    //             .values(&new_project_records)
    //             .get_result::<ProjectRecord>(conn)
    //     })
    //     .await;

    Ok(project_record)
}

pub async fn update(
    db: &Db,
    id: i32,
    new_project_records: NewProjectRecord,
) -> Result<ProjectRecord, sqlx::Error> {
    let project_record = sqlx::query_as!(ProjectRecord,
        "UPDATE project_records SET project_id = $1, record_id = $2 WHERE id = $3 RETURNING *",
        new_project_records.project_id, new_project_records.record_id, id)
        .fetch_one(&db.0)
        .await?;
    // let project_records = db
    //     .run(move |conn| {
    //         diesel::update(project_records::table)
    //             .filter(project_records::id.eq(id))
    //             .set(new_project_records)
    //             .get_result::<ProjectRecord>(conn)
    //     })
    //     .await;

    Ok(project_record)
}
