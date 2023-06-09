use diesel::prelude::*;

use crate::database::connection::Db;
use crate::database::schema::records;
use crate::database::schema::project_records;

use crate::app::modules::records::model::{NewRecord, Record};

pub async fn get_all(db: &Db) -> Result<Vec<Record>, diesel::result::Error> {
    let records = db
        .run(move |conn| records::table.load::<Record>(conn))
        .await;

    records
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Record, diesel::result::Error> {
    let record = db
        .run(move |conn| {
            records::table
                .filter(records::id.eq(id))
                .first::<Record>(conn)
        })
        .await;

    record
}

pub async fn get_by_multiple_ids(
    db: &Db,
    ids: Vec<i32>,
) -> Result<Vec<Record>, diesel::result::Error> {
    let records = db
        .run(move |conn| {
            records::table
                .filter(records::id.eq_any(ids))
                .load::<Record>(conn)
        })
        .await;

    records
}

pub async fn get_all_by_user_id(
    db: &Db,
    user_id: i32,
) -> Result<Vec<Record>, diesel::result::Error> {
    let records = db
        .run(move |conn| {
            records::table
                .filter(records::user_id.eq(user_id))
                .order(records::id.desc())
                .load::<Record>(conn)
        })
        .await;

    records
}

pub async fn get_last_by_user_id(
    db: &Db,
    user_id: i32,
) -> Result<Record, diesel::result::Error> {
    let record = db
        .run(move |conn| {
            records::table
                .filter(records::user_id.eq(user_id))
                .order(records::id.desc())
                .first::<Record>(conn)
        })
        .await;

    record
}

pub async fn get_last_of_every_user_by_project_id(db: &Db, id: i32) -> Result<Vec<Record>, diesel::result::Error> {
    let records = db
        .run(move |conn| {
            records::table
                .distinct_on(records::user_id)
                .inner_join(project_records::table)
                .filter(project_records::project_id.eq(id))
                .order((records::user_id, records::id.desc()))
                .select(records::all_columns)
                .load::<Record>(conn)
        })
        .await;

    records
}

pub async fn create(db: &Db, new_record: NewRecord) -> Result<Record, diesel::result::Error> {
    // make default value {}
    let record = match new_record.record {
        Some(record) => record,
        None => rocket::serde::json::json!({}),
    };

    let new_record = NewRecord {
        user_id: new_record.user_id,
        record: Some(record),
    };

    let record = db
        .run(move |conn| {
            diesel::insert_into(records::table)
                .values(&new_record)
                .get_result::<Record>(conn)
        })
        .await;

    record
}

pub async fn update(
    db: &Db,
    id: i32,
    new_record: NewRecord,
) -> Result<Record, diesel::result::Error> {
    let record = db
        .run(move |conn| {
            diesel::update(records::table)
                .filter(records::id.eq(id))
                .set(new_record)
                .get_result::<Record>(conn)
        })
        .await;

    record
}
