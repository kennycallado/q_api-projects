use serde::{Deserialize, Serialize};

use crate::database::schema::project_values;


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[diesel(table_name = project_values)]
#[serde(crate = "rocket::serde")]
pub struct ProjectValue {
    pub id: i32,
    pub project_id: i32,
    pub values_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = project_values)]
#[serde(crate = "rocket::serde")]
pub struct NewProjectValue {
    pub project_id: i32,
    pub values_id: i32,
}

impl From<ProjectValue> for NewProjectValue {
    fn from(value: ProjectValue) -> Self {
        Self {
            project_id: value.project_id,
            values_id: value.values_id,
        }
    }
}
