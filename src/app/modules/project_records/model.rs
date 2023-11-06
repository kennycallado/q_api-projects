use serde::{Deserialize, Serialize};

#[cfg(feature = "db_diesel")]
use crate::database::schema::project_records;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;
#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

#[cfg_attr(feature = "db_diesel", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "db_diesel", diesel(table_name = project_records))]
#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProjectRecord {
    pub id: i32,
    pub project_id: i32,
    pub record_id: i32,
}

#[cfg_attr(feature = "db_diesel", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "db_diesel", diesel(table_name = project_records))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewProjectRecord {
    pub project_id: i32,
    pub record_id: i32,
}

impl From<ProjectRecord> for NewProjectRecord {
    fn from(value: ProjectRecord) -> Self {
        Self {
            project_id: value.project_id,
            record_id: value.record_id,
        }
    }
}
