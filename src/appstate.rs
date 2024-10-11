

#[derive(Clone)]
pub struct AppState {
    pub version_hash: String,
}

impl AppState {
    pub fn new() -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Self { version_hash: timestamp.to_string() }
    }
}


