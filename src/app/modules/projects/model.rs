use serde::{Deserialize, Serialize};

#[cfg(feature = "db_diesel")]
use crate::database::schema::projects;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;
#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::app::modules::records::model::Record;

#[cfg_attr(feature = "db_diesel", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub keys: Option<Vec<String>>,
}

#[cfg_attr(feature = "db_diesel", derive(Insertable, AsChangeset))]
#[cfg_attr(feature = "db_diesel", diesel(table_name = projects))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewProject {
    pub name: String,
    pub keys: Option<Vec<String>>,
}

impl From<Project> for NewProject {
    fn from(project: Project) -> Self {
        NewProject {
            name: project.name,
            keys: project.keys,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProjectWithRecords {
    pub id: i32,
    pub name: String,
    pub keys: Option<Vec<String>>,
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
