use askama::Template;
use axum::response::{Html, IntoResponse};
use tracing::error;

use crate::error::Error;



pub async fn index() -> impl IntoResponse {
    let index = IndexTemplate;
    index.render().map_err(render_failure).map(|u| Html(u))
}

#[derive(Template)] 
#[template(path = "index.html")]

struct IndexTemplate;

pub fn render_failure(err: askama::Error) -> Error {
    error!("Template rendering failed: {}", err);
    Error::InternalServerError
}