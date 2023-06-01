use serde::{Deserialize, Serialize};

use crate::database::schema::projects;

use crate::app::modules::records::model::Record;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub keys: Vec<Option<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = projects)]
#[serde(crate = "rocket::serde")]
pub struct NewProject {
    pub name: String,
    pub keys: Vec<String>,
}

impl From<Project> for NewProject {
    fn from(project: Project) -> Self {
        NewProject {
            name: project.name,
            keys: project.keys.into_iter().filter_map(|k| k).collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProjectWithRecords {
    pub id: i32,
    pub name: String,
    pub keys: Vec<Option<String>>,
    pub records: Option<Vec<Record>>,
}

// #[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
// #[serde(crate = "rocket::serde")]
// pub struct ProjectWithRecordsAndUser {
//     pub id: i32,
//     pub user_id: i32,
//     pub name: String,
//     pub keys: Vec<Option<String>>,
//     pub records: Option<Vec<Records>>,
// }
