pub mod person;

use axum::Json;
use axum::routing::get;
use axum::Router;

use crate::routes::people::people;




pub async fn api_router() -> Router {
    Router::new()
        .route("/people", get(people))
}
