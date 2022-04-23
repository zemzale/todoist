use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod todoist;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = todoist::Client{http_client: reqwest::Client::new()};

    let tasks = client.find().await.unwrap();

    for task in tasks.iter() {
        println!("{} | {} ", task.id, task.content)
    }

    Ok(())
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long)]
        list: bool,
    },
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long, parse(from_occurrences))]
    debug: usize,

    #[clap(subcommand)]
    command: Option<Commands>,
}
