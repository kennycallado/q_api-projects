use serde::{Deserialize, Serialize};

use crate::database::schema::values;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[diesel(table_name = values)]
#[serde(crate = "rocket::serde")]
pub struct Value {
    pub id: i32,
    pub user_id: i32,
    pub value: rocket::serde::json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = values)]
#[serde(crate = "rocket::serde")]
pub struct NewValue {
    pub user_id: i32,
    pub value: rocket::serde::json::Value,
}

impl From<Value> for NewValue {
    fn from(value: Value) -> Self {
        Self {
            user_id: value.user_id,
            value: value.value,
        }
    }
}
