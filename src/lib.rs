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
