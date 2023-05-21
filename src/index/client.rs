use super::articles::Article;
use crate::config::Config;
use futures::executor::block_on;
use meilisearch_sdk::errors::Error;
use meilisearch_sdk::key::{Action, KeyBuilder, KeysQuery, Key};
use meilisearch_sdk::search::SearchResults;

// The name if the index in Meilisearch.
const INDEX_NAME: &str = "rukby";
// The name of the API key used.
const API_KEY_NAME: &str = "rukby";

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

    pub fn search(&self, query: &str) -> Vec<Article> {
        return block_on(async move {
            let mut v: Vec<Article> = Vec::new();
            let response: Result<SearchResults<Article>, Error> = self
                .client
                .index(INDEX_NAME)
                .search()
                .with_query(query)
                .execute()
                .await;
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

// Administration-type functions
impl Client {
    pub fn for_master_key(config: &Config, master_key: String) -> Self {
        Client {
            client: meilisearch_sdk::Client::new(config.meilisearch_host(), Some(master_key)),
        }
    }

    pub fn ensure_index(&self) {
        block_on(async move {
            match self.client.get_index(INDEX_NAME).await {
                Ok(_) => (),
                Err(_) => {
                    self.client
                        .create_index(INDEX_NAME, Some("id"))
                        .await
                        .unwrap();
                }
            }
        });
    }

    pub fn ensure_api_key(&self) -> String {
        return block_on(async move {
            let rv = KeysQuery::new()
                .with_limit(50)
                .execute(&self.client)
                .await
                .unwrap();
            for key in rv.results {
                if key.name.unwrap() == API_KEY_NAME {
                    return key.key;
                }
            }
            self.create_api_key().await.unwrap().key
        });
    }

    async fn create_api_key(&self) -> Result<Key, Error> {
        let actions = vec![Action::DocumentsAdd, Action::DocumentsGet, Action::Search];
        let mut key_options = KeyBuilder::new();
        key_options
            .with_actions(actions)
            .with_name(API_KEY_NAME)
            .with_description("API key used by rukby")
            .with_index(INDEX_NAME);
        self.client.create_key(key_options).await
    }
}
