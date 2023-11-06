<<<<<<< HEAD
use crate::app::server::rocket;
use rocket::http::{Accept, ContentType, Status};

#[rocket::async_test]
async fn test_health() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(rocket().await).await.unwrap();
=======
use super::*;
use rocket::http::{Accept, ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn test_health() {
    let client = Client::tracked(app::server::rocket()).unwrap();
>>>>>>> 7e9a26c (Initial commit)
    let response = client
        .get("/health")
        .header(Accept::JSON)
        // .header(Header::new("Authorization", format!("Bearer {bearer}")))
        .header(ContentType::JSON)
<<<<<<< HEAD
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await, Some("OK".into()));
=======
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("OK".into()));
>>>>>>> 7e9a26c (Initial commit)
}
