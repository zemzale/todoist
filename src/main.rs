mod survey;
mod api;
mod config;

use clap::{Args, Parser, Subcommand};
use crate::survey::Question;
use crate::config::setup_config;

extern crate dirs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = setup_config()?;
    let client = api::Client::new(reqwest::Client::new(), config.api_key);

    let cli = Cli::parse();
    match &cli.command {
        Commands::Tasks(tasks) => match &tasks.command {
            TaskCommands::List { filter } => {
                match client
                    .find(Some(api::TaskFilter {
                        day_filter: filter.to_owned().unwrap_or(String::from("(today|overdue)")),
                    }))
                    .await
                {
                    Ok(resp) => {
                        for task in resp.iter() {
                            println!("{} | {}", task.id.unwrap(), task.content)
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            TaskCommands::Create { content, due } => {
                let task_content: String;
                let task_due: String;

                if content.is_none() {
                    task_content = survey::ask(Question::new(String::from("Your tasks name")));
                } else {
                    task_content = content.to_owned().unwrap();
                }

                if due.is_none() {
                    task_due = survey::ask(Question::new(String::from("Due date")));
                } else {
                    task_due = due.to_owned().unwrap();
                }

                match client    
                    .create(api::Task::new(task_content.to_owned(), task_due.to_owned()))
                    .await
                {
                    Ok(task) => {
                        println!("{}", task.content)
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            TaskCommands::Done { id } => match client.close(*id).await {
                Ok(_) => println!("task done"),
                Err(e) => {
                    println!("{}", e);
                }
            },
        },
    }

    Ok(())
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Work with tasks
    Tasks(Tasks),
}

#[derive(Debug, Args)]
struct Tasks {
    #[clap(subcommand)]
    command: TaskCommands,
}

#[derive(Debug, Subcommand)]
enum TaskCommands {
    // List commands, default to today and overdue
    List {
        #[clap(long, short)]
        filter: Option<String>,
    },
    // Create a task
    Create {
        // Content of the task
        content: Option<String>,
        // Tasks due date
        due: Option<String>,
    },
    // Mark task as done
    Done {
        // ID of the task
        id: i64,
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
