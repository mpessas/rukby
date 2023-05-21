use clap::{Parser, Subcommand};
use rukby::config::Config;
use rukby::sources::UrlSource;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a document to the index.
    Add {
        /// the document to add
        url: String,
        /// tags for the document
        #[arg(short, long)]
        tags: Vec<String>,
    },
    /// Searches for documents matching a query
    Search {
        /// the query
        query: String,
    },
    /// Initialize the database.
    Init {
        /// The api key to use to initialize the project.
        #[arg(long)]
        master_key: String,
    },
}

fn main() {
    let config = Config::from_environment();
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Add { url, tags }) => {
            let source = UrlSource::from_url(String::from(url));
            rukby::add_content(&config, &source, &tags);
        }
        Some(Commands::Search { query }) => rukby::search(&config, &query),
        Some(Commands::Init { master_key }) => rukby::init(&config, master_key.to_string()),
        None => {}
    }
}
