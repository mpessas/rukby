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
