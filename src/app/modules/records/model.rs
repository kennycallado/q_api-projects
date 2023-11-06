use serde::{Deserialize, Serialize};

#[cfg(feature = "db_diesel")]
use crate::database::schema::records;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;
#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

#[cfg_attr(feature = "db_diesel", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "db_diesel", diesel(table_name = records))]
#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Record {
    pub id: i32,
    pub user_id: i32,
    pub record: rocket::serde::json::Value,
}

#[cfg_attr(feature = "db_diesel", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "db_diesel", diesel(table_name = records))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewRecord {
    pub user_id: i32,
    pub record: Option<rocket::serde::json::Value>,
}

impl From<Record> for NewRecord {
    fn from(record: Record) -> Self {
        Self {
            user_id: record.user_id,
            record: Some(record.record),
        }
    }
}
