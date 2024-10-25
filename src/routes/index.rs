use askama_axum::Template;
use axum::response::{Html, IntoResponse};


use crate::api::person::{all_people, Person};

use super::render_failure;



pub async fn index() -> impl IntoResponse {

    let index = IndexTemplate::new().await;
    index.render().map_err(render_failure).map(|u| Html(u))
}

#[derive(Template)] 
#[template(path = "index.html")]

struct IndexTemplate {
    projects: Vec<Project>,
    people: Vec<Person>,
}

impl IndexTemplate {
    async fn new() -> Self {

        let projects = vec![
            Project { title: "Exoskeleton".to_string(), slug: "exoskeleton".to_string(), description: "Cutting-edge exoskeleton technology".to_string() },
            Project { title: "Lithography".to_string(), slug: "lithography".to_string(), description: "Advanced nanoscale lithography techniques".to_string() },
            Project { title: "Plasma Jet".to_string(), slug: "plasma_jet".to_string(), description: "Innovative plasma jet applications".to_string() },
            Project { title: "Energy Storage".to_string(), slug: "energy_storage".to_string(), description: "Next-generation energy storage solutions".to_string() },
            Project { title: "Electrowetting".to_string(), slug: "electrowetting".to_string(), description: "Electrowetting research and applications".to_string() },
            Project { title: "Drone".to_string(), slug: "drone".to_string(), description: "Advanced drone technology development".to_string() },
            Project { title: "Biosensor".to_string(), slug: "biosensor".to_string(), description: "Cutting edge biosensor research".to_string() },
            Project { title: "Analytics".to_string(), slug: "analytics".to_string(), description: "Advanced data analytics for nanotech".to_string() },
        ];

        let mut people = all_people().await;

        let non_included = people.split_off(6);

        Self { projects, people }
    }
}

struct Project {
    title: String,
    slug: String,
    description: String,
}

