use readability::{error, extractor};

#[derive(Debug)]
pub struct Content {
    text: String,
}

impl Content {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug)]
pub struct UrlSource {
    url: String,
}

impl UrlSource {
    pub fn from_url(url: String) -> Self {
        Self { url: url }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn fetch(&self) -> Result<Content, error::Error> {
        let text = extractor::scrape(&self.url)?.content;
        Ok(Content::new(text))
    }
}
