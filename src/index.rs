use crate::config::Config;
use rand::{distributions::Alphanumeric, Rng};
use futures::executor::block_on;
use serde::{Serialize, Deserialize};

// The size of the ID keys used in the index.
const ID_SIZE: usize = 9;

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
        let task = self.client
            .index("rukby")
            .add_documents(&[article], None)
            .await
                .unwrap();
            println!("{:?}", task);
        });
    }

}

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
}



fn generate_id() -> String {
    let id: String = rand::thread_rng().sample_iter(&Alphanumeric).take(ID_SIZE).map(char::from).collect();
    id.to_lowercase()
}
