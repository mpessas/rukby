use crate::config::Config;
use meilisearch_sdk::search::SearchResults;
use meilisearch_sdk::errors::Error;
use rand::{distributions::Alphanumeric, Rng};
use futures::executor::block_on;
use serde::{Serialize, Deserialize};

// The size of the ID keys used in the index.
const ID_SIZE: usize = 9;
// The name if the index in Meilisearch.
const INDEX_NAME: &str = "rukby";
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
            tags

        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn excerpt(&self) -> &str {
        &self.content[..EXCERPT_LENGTH]
    }
}


fn generate_id() -> String {
    let id: String = rand::thread_rng().sample_iter(&Alphanumeric).take(ID_SIZE).map(char::from).collect();
    id.to_lowercase()
}


pub struct Client {
    client: meilisearch_sdk::Client,
}

impl Client {
    pub fn new(config: &Config) -> Self {
        Client {
            client: meilisearch_sdk::Client::new(
                config.meilisearch_host(),
                config.meilisearch_api_key(),
            ),
        }
    }

    pub fn add_article(&self, article: &Article) {
        block_on(async move {
        self.client
            .index(INDEX_NAME)
            .add_documents(&[article], None)
            .await
                .unwrap();
        });
    }

    pub fn search(&self, query: &str) -> Vec<Article>{
        return block_on(async move {
            let mut v: Vec<Article> = Vec::new();
            let response: Result<SearchResults<Article>, Error> = self.client.index(INDEX_NAME).search().with_query(query).execute().await;
            match response {
                Ok(results) => {
                    println!("Found {0} results", results.hits.len());
                    if results.hits.is_empty() {
                        return v;
                    }
                    for e in results.hits {
                        v.push(e.result);
                    }
                    return v;
                }
                Err(e) => {
                    println!("Error {:?}", e);
                    return Vec::new();
                }
            }
        });
    }
}
