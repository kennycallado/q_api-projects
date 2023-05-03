use crate::app::modules::projects::controller::routes as projects_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket.mount("/api/v1/project", projects_routes())
    })
}
