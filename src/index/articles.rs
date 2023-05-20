use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;

// The size of the ID keys used in the index.
const ID_SIZE: usize = 9;
// The length of the excerpts used.
const EXCERPT_LENGTH: usize = 300;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    id: String,
    content: String,
    url: String,
    tags: Vec<String>,
}

impl Article {
    pub fn new(content: String, url: String, tags: Vec<String>) -> Self {
        Article {
            id: generate_id(),
            content,
            url,
            tags,
        }
    }

    pub fn excerpt(&self) -> &str {
        let size = if EXCERPT_LENGTH > self.content.len() {
            self.content.len()
        } else {
            EXCERPT_LENGTH
        };
        &self.content[..size]
    }
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.excerpt())
    }
}

fn generate_id() -> String {
    let id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(ID_SIZE)
        .map(char::from)
        .collect();
    id.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generates_article_with_id() {
        let article = Article::new(
            "Some text".to_string(),
            "https://example.com/article".to_string(),
            vec!["tag1".to_string(), "tag2".to_string()],
        );
        assert_eq!(article.id.len(), ID_SIZE);
    }

    #[test]
    fn excerpt_returns_substring() {
        let article = Article::new(
            "Some text".to_string().repeat(500),
            "https://example.com/article".to_string(),
            vec!["tag1".to_string(), "tag2".to_string()],
        );
        assert_eq!(article.excerpt().len(), EXCERPT_LENGTH);
    }

    #[test]
    fn display_trait() {
        let article = Article::new(
            "Some text".to_string(),
            "https://example.com/article".to_string(),
            vec!["tag1".to_string(), "tag2".to_string()],
        );
        assert_eq!(article.to_string(), format!("{}: Some text", article.id));
    }
}
