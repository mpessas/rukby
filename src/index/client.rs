use meilisearch_sdk::errors::Error;
use meilisearch_sdk::search::SearchResults;
use futures::executor::block_on;
use crate::config::Config;
use super::articles::Article;

// The name if the index in Meilisearch.
const INDEX_NAME: &str = "rukby";


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
