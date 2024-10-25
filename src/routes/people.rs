use askama_axum::Template;
use axum::response::{Html, IntoResponse};
use serde::{Serialize, Deserialize};

use crate::api::person::Person;

use super::render_failure;

pub async fn people() -> impl IntoResponse {
    let people = PeopleTemplate::new().await;
    people.render().map_err(render_failure).map(|u| Html(u))
}

#[derive(Template)]
#[template(path = "people.html")]
struct PeopleTemplate {
    people: Vec<Person>,
}

impl PeopleTemplate {
    async fn new() -> Self {
        let people = crate::api::person::all_people().await;
        Self { people }
    }
}
