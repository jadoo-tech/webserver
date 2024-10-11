use askama_axum::Template;
use axum::response::{Html, IntoResponse};
use serde::{Serialize, Deserialize};

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
        let people = crate::api::all_people().await;
        Self { people }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    name: String,
    role: String,
    image: String,
}

impl Person {
    pub fn new(name: &str, role: &str, image: &str) -> Self {
        Self { name: name.to_string(), role: role.to_string(), image: image.to_string() }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
