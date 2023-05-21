pub mod config;
mod index;
pub mod sources;
use crate::config::Config;
use crate::index::{Article, Client};
use crate::sources::UrlSource;

pub fn add_content(config: &Config, source: &UrlSource, tags: &Vec<String>) {
    let client = Client::new(&config);
    let content = source.fetch().unwrap();
    // println!("{}", content.text());
    let article = Article::new(
        content.text().to_string(),
        String::from(source.url()),
        tags.to_vec(),
    );
    client.add_article(&article);
}

pub fn search(config: &Config, query: &str) {
    let client = Client::new(&config);
    let results = client.search(query);
    println!("Found {0} results", results.len());
    for r in results {
        println!("{}", r);
    }
}

pub fn init(config: &Config, master_key: String) {
    let client = Client::for_master_key(config, master_key);
    print!("Creating the index...");
    client.ensure_index();
    println!("Done");

    print!("Generating an API key...");
    let api_key = client.ensure_api_key();
    println!("Done. The API key is {}", api_key);
}
