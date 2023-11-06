#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::database::connection::Db;

use crate::app::modules::records::model::{NewRecord, Record};

pub async fn get_all(db: &Db) -> Result<Vec<Record>, sqlx::Error> {
    let records = sqlx::query_as!(Record, "SELECT * FROM records")
        .fetch_all(&db.0)
        .await?;

    // let records = db
    //     .run(move |conn| records::table.load::<Record>(conn))
    //     .await;

    Ok(records)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Record, sqlx::Error> {
    let record = sqlx::query_as!(Record, "SELECT * FROM records WHERE id = $1", id)
        .fetch_one(&db.0)
        .await?;

    // let record = db
    //     .run(move |conn| {
    //         records::table
    //             .filter(records::id.eq(id))
    //             .first::<Record>(conn)
    //     })
    //     .await;

    Ok(record)
}

pub async fn get_by_multiple_ids(
    db: &Db,
    ids: Vec<i32>,
) -> Result<Vec<Record>, sqlx::Error> {
    let records = sqlx::query_as!(Record, "SELECT * FROM records WHERE id = ANY($1)", &ids)
        .fetch_all(&db.0)
        .await?;

    // let records = db
    //     .run(move |conn| {
    //         records::table
    //             .filter(records::id.eq_any(ids))
    //             .load::<Record>(conn)
    //     })
    //     .await;

    Ok(records)
}

pub async fn get_all_by_user_id(
    db: &Db,
    user_id: i32,
) -> Result<Vec<Record>, sqlx::Error> {
    let records = sqlx::query_as!(Record, "SELECT * FROM records WHERE user_id = $1 ORDER BY user_id DESC", user_id)
        .fetch_all(&db.0)
        .await?;

    // let records = db
    //     .run(move |conn| {
    //         records::table
    //             .filter(records::user_id.eq(user_id))
    //             .order(records::id.desc())
    //             .load::<Record>(conn)
    //     })
    //     .await;

    Ok(records)
}

pub async fn get_last_by_user_id(
    db: &Db,
    user_id: i32,
) -> Result<Record, sqlx::Error> {
    let record = sqlx::query_as!(Record, "SELECT * FROM records WHERE user_id = $1 ORDER BY user_id DESC LIMIT 1", user_id)
        .fetch_one(&db.0)
        .await?;

    // let record = db
    //     .run(move |conn| {
    //         records::table
    //             .filter(records::user_id.eq(user_id))
    //             .order(records::id.desc())
    //             .first::<Record>(conn)
    //     })
    //     .await;

    Ok(record)
}

pub async fn get_last_of_every_user_by_project_id(db: &Db, id: i32) -> Result<Vec<Record>, sqlx::Error> {
    let records = sqlx::query_as!(Record,
        r#"
        SELECT DISTINCT ON (records.user_id) records.*
        FROM records
        INNER JOIN project_records ON project_records.record_id = records.id
        WHERE project_records.project_id = $1
        ORDER BY records.user_id, records.id DESC
        "#, id)
        .fetch_all(&db.0)
        .await?;

    // let records = db
    //     .run(move |conn| {
    //         records::table
    //             .distinct_on(records::user_id)
    //             .inner_join(project_records::table)
    //             .filter(project_records::project_id.eq(id))
    //             .order((records::user_id, records::id.desc()))
    //             .select(records::all_columns)
    //             .load::<Record>(conn)
    //     })
    //     .await;

    Ok(records)
}

pub async fn create(db: &Db, new_record: NewRecord) -> Result<Record, sqlx::Error> {
    // make default value {}
    // let record = match new_record.record {
    //     Some(record) => record,
    //     None => rocket::serde::json::json!({}),
    // };

    // let new_record = NewRecord {
    //     user_id: new_record.user_id,
    //     record: Some(record),
    // };

    // let record = db
    //     .run(move |conn| {
    //         diesel::insert_into(records::table)
    //             .values(&new_record)
    //             .get_result::<Record>(conn)
    //     })
    //     .await;

    let record = sqlx::query_as!(
        Record,
        "INSERT INTO records (user_id, record) VALUES ($1, $2) RETURNING *",
        new_record.user_id, new_record.record)
        .fetch_one(&db.0)
        .await?;

    Ok(record)
}

pub async fn update(
    db: &Db,
    id: i32,
    new_record: NewRecord,
) -> Result<Record, sqlx::Error> {
    let record = sqlx::query_as!(
        Record,
        "UPDATE records SET record = $1 WHERE id = $2 RETURNING *",
        new_record.record, id).fetch_one(&db.0).await?;

    // let record = db
    //     .run(move |conn| {
    //         diesel::update(records::table)
    //             .filter(records::id.eq(id))
    //             .set(new_record)
    //             .get_result::<Record>(conn)
    //     })
    //     .await;

    Ok(record)
}
