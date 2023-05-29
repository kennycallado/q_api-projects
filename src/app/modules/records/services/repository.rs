use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::records;

use crate::app::modules::records::model::{Record, NewRecord};

pub async fn get_all(db: &Db) -> Result<Vec<Record>, diesel::result::Error> {
    let records = db.run(move |conn| {
        records::table
            .load::<Record>(conn)
    }).await;

    records
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Record, diesel::result::Error> {
    let record = db.run(move |conn| {
        records::table
            .filter(records::id.eq(id))
            .first::<Record>(conn)
    }).await;

    record
}

pub async fn get_all_by_user_id(db: &Db, user_id: i32) -> Result<Vec<Record>, diesel::result::Error> {
    let records = db.run(move |conn| {
        records::table
            .filter(records::user_id.eq(user_id))
            .order(records::id.desc())
            .load::<Record>(conn)
    }).await;

    records
}

pub async fn get_last_by_user_id(db: &Db, user_id: i32) -> Result<Record, diesel::result::Error> {
    let record = db.run(move |conn| {
        records::table
            .filter(records::user_id.eq(user_id))
            .order(records::id.desc())
            .first::<Record>(conn)
    }).await;

    record
}

pub async fn create(db: &Db, new_record: NewRecord) -> Result<Record, diesel::result::Error> {
    let record = db.run(move |conn| {
        diesel::insert_into(records::table)
            .values(&new_record)
            .get_result::<Record>(conn)
    }).await;

    record
}

pub async fn update(db: &Db, id: i32, new_record: NewRecord) -> Result<Record, diesel::result::Error> {
    let record = db.run(move |conn| {
        diesel::update(records::table)
            .filter(records::id.eq(id))
            .set(new_record)
            .get_result::<Record>(conn)
    }).await;

    record
}
