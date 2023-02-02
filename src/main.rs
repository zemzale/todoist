mod api;
mod config;

use std::io;

use crate::config::setup_config;
use clap::{Args, Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use dialoguer::theme::ColorfulTheme;
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
    let theme = ColorfulTheme::default();

    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        eprintln!("Generating completion file for {generator:?}...");
        print_completions(generator, &mut cmd);
    } else {
        if let Some(command) = cli.command {
            match command {
                Commands::Tasks(tasks) => match &tasks.command {
                    TaskCommands::List { filter, raw } => {
                        let resp = client
                            .find(Some(api::TaskFilter {
                                day_filter: filter
                                    .to_owned()
                                    .unwrap_or(String::from("today|overdue")),
                            }))
                            .await;
                        match resp {
                            Ok(resp) => {
                                let mut output_rows: Vec<Vec<String>> = Vec::new();

                                for task in resp.iter() {
                                    let project_name = task.project(&client).await.unwrap().name;
                                    output_rows.push(vec![
                                        task.id.to_owned(),
                                        project_name,
                                        task.content.to_owned(),
                                        task.priority.to_string(),
                                    ]);
                                }

                                if raw.unwrap_or(false) {
                                    for row in output_rows {
                                        for field in row {
                                            print!("{},", field);
                                        }
                                        println!();
                                    }
                                } else {
                                    let mut table = Table::new();
                                    table
                                        .set_header(vec!["ID", "Project", "Task name", "Priority"])
                                        .load_preset(UTF8_FULL)
                                        .apply_modifier(UTF8_ROUND_CORNERS);

                                    table.add_rows(output_rows);
                                    println!("{table}");
                                }
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
                        labels,
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
                        .labels(if labels.len() > 0 {
                            labels.to_owned()
                        } else {
                            let items = client.label_list().await?;

                            let selected_labels = dialoguer::MultiSelect::new()
                                .with_prompt("Lables:")
                                .items(
                                    &items
                                        .iter()
                                        .map(|x| -> String { x.name.to_owned() })
                                        .collect::<Vec<String>>(),
                                )
                                .interact()
                                .unwrap();

                            let mut labels: Vec<String> = Vec::new();

                            for i in selected_labels {
                                labels.push(items[i].name.to_owned());
                            }
                            labels
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
                    ProjectCommands::View { id } => match client.project_view(id.to_string()).await
                    {
                        Ok(project) => {
                            println!("{} | {}", project.id, project.name);
                        }
                        Err(e) => println!("{}", e),
                    },
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
