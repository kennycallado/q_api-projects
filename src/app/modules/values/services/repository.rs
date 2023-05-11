use diesel::prelude::*;

use crate::config::database::Db;
use crate::database::schema::values;

use crate::app::modules::values::model::{Value, NewValue};

pub async fn get_all(db: &Db) -> Result<Vec<Value>, diesel::result::Error> {
    let values = db.run(move |conn| {
        values::table
            .load::<Value>(conn)
    }).await;

    values
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Value, diesel::result::Error> {
    let value = db.run(move |conn| {
        values::table
            .filter(values::id.eq(id))
            .first::<Value>(conn)
    }).await;

    value
}

pub async fn get_all_by_user_id(db: &Db, user_id: i32) -> Result<Vec<Value>, diesel::result::Error> {
    let values = db.run(move |conn| {
        values::table
            .filter(values::user_id.eq(user_id))
            .order(values::id.desc())
            .load::<Value>(conn)
    }).await;

    values
}

pub async fn get_last_by_user_id(db: &Db, user_id: i32) -> Result<Value, diesel::result::Error> {
    let value = db.run(move |conn| {
        values::table
            .filter(values::user_id.eq(user_id))
            .order(values::id.desc())
            .first::<Value>(conn)
    }).await;

    value
}

pub async fn create(db: &Db, new_value: NewValue) -> Result<Value, diesel::result::Error> {
    let value = db.run(move |conn| {
        diesel::insert_into(values::table)
            .values(&new_value)
            .get_result::<Value>(conn)
    }).await;

    value
}

pub async fn update(db: &Db, id: i32, new_value: NewValue) -> Result<Value, diesel::result::Error> {
    let value = db.run(move |conn| {
        diesel::update(values::table)
            .filter(values::id.eq(id))
            .set(new_value)
            .get_result::<Value>(conn)
    }).await;

    value
}
