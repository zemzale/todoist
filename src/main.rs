mod api;
mod config;

use crate::config::setup_config;
use clap::{Args, Parser, Subcommand};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use dialoguer::theme::ColorfulTheme;
use yansi::Paint;

extern crate dirs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = setup_config()?;
    let client = api::Client::new(reqwest::Client::new(), config.api_key);

    let cli = Cli::parse();
    let theme = ColorfulTheme::default();
    match &cli.command {
        Commands::Tasks(tasks) => match &tasks.command {
            TaskCommands::List { filter } => {
                let resp = client
                    .find(Some(api::TaskFilter {
                        day_filter: filter.to_owned().unwrap_or(String::from("today")),
                    }))
                    .await;
                match resp {
                    Ok(resp) => {
                        let mut table = Table::new();
                        table
                            .set_header(vec!["ID", "Project", "Task name", "Priority"])
                            .load_preset(UTF8_FULL)
                            .apply_modifier(UTF8_ROUND_CORNERS);

                        for task in resp.iter() {
                            table.add_row(vec![
                                &task.id,
                                &client
                                    .project_view(task.project_id.to_string())
                                    .await
                                    .unwrap()
                                    .name,
                                &task.content,
                                &task.priority.to_string(),
                            ]);
                        }

                        println!("{table}");
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            TaskCommands::Create {
                content,
                due,
                project,
            } => {
                let prompt = |prompt: &str| -> String {
                    dialoguer::Input::with_theme(&theme)
                        .with_prompt(prompt)
                        .interact()
                        .expect("failed to get input from promt")
                };

                let task_create = api::TaskCreate::new(if let Some(x) = content {
                    x.to_owned()
                } else {
                    prompt("Your tasks name")
                })
                .project(if let Some(x) = project {
                    x.to_owned()
                } else {
                    let selections = client
                        .project_list()
                        .await
                        .expect("failed to fetch projects");

                    let selected_project = dialoguer::FuzzySelect::with_theme(&theme)
                        .with_prompt("Project:")
                        .items(&selections)
                        .default(0)
                        .interact()
                        .unwrap();

                    selections.get(selected_project).unwrap().id.to_owned()
                })
                .due(if let Some(x) = due {
                    x.to_owned()
                } else {
                    prompt("Due date")
                })
                .to_owned();

                match client.create(task_create).await {
                    Ok(task) => {
                        println!("{}", task.content)
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            TaskCommands::Done { id } => match client.close(id.to_string()).await {
                Ok(_) => println!("task done"),
                Err(e) => {
                    println!("{}", e);
                }
            },
            TaskCommands::View { id } => match client.view(id.to_string()).await {
                Ok(task) => {
                    let project = client.project_view(task.project_id).await.unwrap();

                    println!("Task : {}", Paint::green(task.content));
                    if task.due.is_some() {
                        println!("Due date : {}", Paint::red(task.due.unwrap().date));
                    }
                    println!("Priority : {}", Paint::green(task.priority));
                    println!("Project : {}", Paint::green(project.name));
                    print!("Labels : ");
                    for lable in task.labels {
                        print!("{} ", Paint::magenta(lable))
                    }
                    println!();
                }
                Err(e) => println!("Failed to view the task: {}", e),
            },
        },
        Commands::Projects(cmd) => match &cmd.command {
            ProjectCommands::List {} => match client.project_list().await {
                Ok(projects) => {
                    for project in projects.iter() {
                        println!("{} | {}", project.id, project.name)
                    }
                }
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
    },
    // Create a task
    Create {
        // Content of the task
        content: Option<String>,
        // Tasks due date
        due: Option<String>,
        // Tasks project
        project: Option<String>,
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
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
