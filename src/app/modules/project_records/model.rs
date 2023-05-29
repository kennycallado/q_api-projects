use serde::{Deserialize, Serialize};

use crate::database::schema::project_records;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[diesel(table_name = project_records)]
#[serde(crate = "rocket::serde")]
pub struct ProjectRecord {
    pub id: i32,
    pub project_id: i32,
    pub records_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = project_records)]
#[serde(crate = "rocket::serde")]
pub struct NewProjectRecord {
    pub project_id: i32,
    pub record_id: i32,
}

impl From<ProjectRecord> for NewProjectRecord {
    fn from(value: ProjectRecord) -> Self {
        Self {
            project_id: value.project_id,
            record_id: value.records_id,
        }
    }
}
