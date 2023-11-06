#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::database::connection::Db;

use crate::app::modules::projects::model::{NewProject, Project};

pub async fn get_all(db: &Db) -> Result<Vec<Project>, sqlx::Error> {
    let projects = sqlx::query_as!(Project, "SELECT * FROM projects").fetch_all(&db.0).await?;

    // let projects = db
    //     .run(move |conn| projects::table.load::<(i32, String, Vec<Option<String>>)>(conn))
    //     .await?;

    // let projects = projects
    //     .into_iter()
    //     .map(|(id, name, keys)| Project {
    //         id,
    //         name,
    //         keys: Some(keys
    //             .into_iter()
    //             .filter_map(|key| key)
    //             .collect::<Vec<String>>()),
    //     })
    //     .collect::<Vec<Project>>();

    Ok(projects)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Project, sqlx::Error> {
    let project = sqlx::query_as!(Project, "SELECT * FROM projects WHERE id = $1", id).fetch_one(&db.0).await?;

    // let project = db
    //     .run(move |conn| {
    //         projects::table
    //             .filter(projects::id.eq(id))
    //             .first::<(i32, String, Vec<Option<String>>)>(conn)
    //     })
    //     .await?;

    // let project = Project {
    //     id: project.0,
    //     name: project.1,
    //     keys: Some(project.2.into_iter().filter_map(|key| key).collect::<Vec<String>>()),
    // };

    Ok(project)
}

pub async fn create(
    db: &Db,
    new_project: NewProject,
) -> Result<Project, sqlx::Error> {
    // let keys = if let Some(keys) = new_project.keys {
    //     keys
    // } else {
    //     vec![]
    // };

    let keys = new_project.keys.unwrap_or(vec![]);

    let project = sqlx::query_as!(Project, "INSERT INTO projects (name, keys) VALUES ($1, $2) RETURNING *", new_project.name, &keys).fetch_one(&db.0).await?;

    // let project = db
    //     .run(move |conn| {
    //         diesel::insert_into(projects::table)
    //             .values(&new_project)
    //             .get_result::<(i32, String, Vec<Option<String>>)>(conn)
    //     })
    //     .await?;

    // let project = Project {
    //     id: project.0,
    //     name: project.1,
    //     keys: Some(project.2.into_iter().filter_map(|key| key).collect::<Vec<String>>()),
    // };

    Ok(project)
}

pub async fn update(
    db: &Db,
    id: i32,
    new_project: NewProject,
) -> Result<Project, sqlx::Error> {
    // let keys = if let Some(keys) = new_project.keys {
    //     keys
    // } else {
    //     vec![]
    // };

    let keys = new_project.keys.unwrap_or(vec![]);

    let project = sqlx::query_as!(Project, "UPDATE projects SET name = $1, keys = $2 WHERE id = $3 RETURNING *", new_project.name, &keys, id).fetch_one(&db.0).await?;

    // let project = db
    //     .run(move |conn| {
    //         diesel::update(projects::table)
    //             .filter(projects::id.eq(id))
    //             .set(&new_project)
    //             .get_result::<(i32, String, Vec<Option<String>>)>(conn)
    //     })
    //     .await?;

    // let project = Project {
    //     id: project.0,
    //     name: project.1,
    //     keys: Some(project.2.into_iter().filter_map(|key| key).collect::<Vec<String>>()),
    // };

    Ok(project)
}
