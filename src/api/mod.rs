use axum::Json;
use axum::routing::get;
use axum::Router;

use crate::routes::people::Person;




pub async fn api_router() -> Router {
    Router::new()
        .route("/people", get(people))
}

pub async fn people() -> Json<Vec<Person>> {
    Json(all_people().await)
}

pub async fn all_people() -> Vec<Person> {
    vec![Person::new("John Doe", "Software Engineer", "/static/placeholder.jpg"), Person::new("Jane Doe", "Product Manager", "/static/placeholder.jpg")]
}
