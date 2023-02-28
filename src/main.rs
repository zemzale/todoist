mod api;
mod cmd;
mod config;

use std::io;

use crate::cmd::Cmd;
use crate::config::setup_config;
use clap::{Args, Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use yansi::Paint;

extern crate dirs;

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = setup_config()?;
    let client = api::Client::new(reqwest::Client::new(), config.api_key);

    let cli = Cli::parse();

    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        eprintln!("Generating completion file for {generator:?}...");
        print_completions(generator, &mut cmd);
    } else {
        let cmd = Cmd::new(&client);

        if let Some(command) = cli.command {
            match command {
                Commands::Tasks(tasks) => match &tasks.command {
                    TaskCommands::List { filter, raw } => cmd.tasks.list(filter, raw).await,
                    TaskCommands::Create {
                        content,
                        due,
                        project,
                        labels,
                    } => cmd.tasks.create(content, due, project, labels).await,
                    TaskCommands::Done { id } => cmd.tasks.done(id).await,
                    TaskCommands::View { id } => cmd.tasks.view(id).await,
                },
                Commands::Projects(projects) => match &projects.command {
                    ProjectCommands::List {} => cmd.projects.list().await,
                    ProjectCommands::View { id } => cmd.projects.view(id).await,
                },
            }
        } else {
            match Cli::command().print_help() {
                Ok(_) => {}
                Err(err) => eprintln!("{}", err),
            }
        }
    }

    Ok(())
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Work with tasks
    Tasks(Tasks),
    // Work with projects
    Projects(Projects),
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
        #[clap(long, short)]
        raw: Option<bool>,
    },
    // Create a task
    Create {
        // Content of the task
        content: Option<String>,
        // Tasks due date
        due: Option<String>,
        // Tasks project
        project: Option<String>,
        // Lables to add to task
        labels: Vec<String>,
    },
    // Mark task as done
    Done {
        // ID of the task
        id: String,
    },
    // View task by id
    View {
        id: String,
    },
}

#[derive(Debug, Args)]
struct Projects {
    #[clap(subcommand)]
    command: ProjectCommands,
}

#[derive(Debug, Subcommand)]
enum ProjectCommands {
    // List command
    List {},
    // View project
    View { id: String },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
    // If provided, outputs the completion file for given shell
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,
}
