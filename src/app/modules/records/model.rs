use serde::{Deserialize, Serialize};

use crate::database::schema::records;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[diesel(table_name = records)]
#[serde(crate = "rocket::serde")]
pub struct Record {
    pub id: i32,
    pub user_id: i32,
    pub record: rocket::serde::json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = records)]
#[serde(crate = "rocket::serde")]
pub struct NewRecord {
    pub user_id: i32,
    pub record: rocket::serde::json::Value,
}

impl From<Record> for NewRecord {
    fn from(record: Record) -> Self {
        Self {
            user_id: record.user_id,
            record: record.record,
        }
    }
}
