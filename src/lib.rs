pub mod config;
pub mod sources;

use sources::UrlSource;

pub fn add_content(source: UrlSource) {
    let content = source.fetch().unwrap();
    println!("{}", content.text());
}
