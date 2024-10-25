use std::path::Path;

use axum::Json;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub blurb: String,
    pub image: String,
    pub name: String,
    pub teams: String,
}


impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.clone();
        write!(f, "{}", name)
    }
}



pub async fn people() -> Json<Vec<Person>> {
    Json(all_people().await)
}

pub async fn all_people() -> Vec<Person> {
    read_people_data()
}

#[tokio::test]
pub async fn test_read_people() {
    let people = read_people_data();

    dbg!(people.len());
    dbg!(people.iter().take(5).collect::<Vec<&Person>>());

    assert!(people.len() > 0);
}

pub fn read_people_data() -> Vec<Person> {
    let data_location = Path::new("./people_data.json");

    let contents = match std::fs::read_to_string(data_location) {
        Ok(data) => data,
        Err(e) => {
            log::error!("Failed to read people_data.json: {e:?}");
            return vec![];
        }
    };

    let people_data: Vec<PersonPrototype> = match serde_json::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            log::error!("People data isn't appropraitely formatted: {e:?}");
            return vec![];
        }
    };

    let out_data = people_data.iter().map(|i| i.clone().into()).collect::<Vec<Person>>();

    out_data
} 

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PersonPrototype {
    pub blurb: Option<String>,
    pub image: Option<String>,
    pub name: Option<String>,
    pub teams: Option<String>,
}

impl Into<Person> for PersonPrototype  {
    fn into(self) -> Person {
        Person {
            blurb: self.blurb.unwrap_or(String::new()),
            image: self.image.unwrap_or(String::new()),
            name: self.name.unwrap_or(String::new()),
            teams: self.teams.unwrap_or(String::new()),
        }
    }
}