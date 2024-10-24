


use std::path::Path;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Person {
    blurb: Option<String>,
    image: Option<String>,
    name: Option<String>,
    teams: Option<String>,
}

#[tokio::test]
pub async fn test_read_people() {
    let people = read_people_data();

    // dbg!(people.len());
    // dbg!(people.iter().take(5).collect::<Vec<&Person>>());

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

    let people_data: Vec<Person> = match serde_json::from_str(&contents) {
        Ok(d) => return d,
        Err(e) => {
            log::error!("People data isn't appropraitely formatted: {e:?}");
            return vec![];
        }
    };
} 