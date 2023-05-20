use std::env;

#[derive(Debug)]
pub struct Config {
    // The SQLite database.
    meilisearch_host: String,
    meilisearch_api_key: Option<String>,
}

impl Config {
    pub fn from_environment() -> Self {
        Self {
            meilisearch_host: match env::var("RUKBY_MEILISEARCH_HOST") {
                Ok(path) => path,
                Err(_) => String::from("http://localhost:7700"),
            },
            meilisearch_api_key: match env::var("RUKBY_MEILISEARCH_API_KEY") {
                Ok(api_key) => Some(api_key),
                Err(_) => None,
            },
        }
    }

    pub fn meilisearch_host(&self) -> &str {
        &self.meilisearch_host
    }

    pub fn meilisearch_api_key(&self) -> Option<&str> {
        self.meilisearch_api_key.as_deref()
    }
}
