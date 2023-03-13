use crate::api;

use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use dialoguer::theme::ColorfulTheme;
use yansi::Paint;

pub struct Cmd<'a> {
    pub tasks: Tasks<'a>,
    pub projects: Projects<'a>,
}

impl Cmd<'_> {
    pub fn new(client: &api::Client) -> Cmd {
        Cmd {
            tasks: Tasks { client },
            projects: Projects { client },
        }
    }
}

pub struct Tasks<'a> {
    client: &'a api::Client,
}

impl Tasks<'_> {
    pub async fn list(&self, filter: &Option<String>, raw: &Option<bool>) {
        let resp = self
            .client
            .find(Some(api::TaskFilter {
                day_filter: Some(filter.to_owned().unwrap_or(String::from("today|overdue"))),
            }))
            .await;
        match resp {
            Ok(resp) => {
                let mut output_rows: Vec<Vec<String>> = Vec::new();

                for task in resp.iter() {
                    let project_name = task.project(&self.client).await.unwrap().name;
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

    pub async fn create(
        &self,
        content: &Option<String>,
        due: &Option<String>,
        project: &Option<String>,
        labels: &Vec<String>,
        priority: &Option<u8>,
    ) {
        let theme = ColorfulTheme::default();
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
            let selections = self
                .client
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
            let items = self.client.label_list().await.unwrap();

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
        .priority(if let Some(x) = priority {
            x.to_owned()
        } else {
            prompt("Priority").parse().unwrap()
        })
        .to_owned();

        let new_task = self.client.create(task_create).await.unwrap();
        println!("{}", new_task.content);
    }

    pub async fn done(&self, id: &Option<String>) {
        if let Some(x) = id {
            match self.client.close(x).await {
                Ok(_) => println!("task done"),
                Err(e) => {
                    println!("{}", e);
                }
            }
        } else {
            let theme = ColorfulTheme::default();
            let selections = self
                .client
                .find(Some(api::TaskFilter {
                    day_filter: Some(String::from("today")),
                }))
                .await
                .expect("failed to fetch tasks");

            let selected_task = dialoguer::FuzzySelect::with_theme(&theme)
                .with_prompt("Task:")
                .items(
                    &selections
                        .iter()
                        .map(|x| -> String { x.content.to_owned() })
                        .collect::<Vec<String>>(),
                )
                .default(0)
                .interact()
                .unwrap();

            self.client
                .close(&selections[selected_task].id)
                .await
                .unwrap();
        }
    }

    pub async fn view(&self, id: &String) {
        match self.client.view(id.to_string()).await {
            Ok(task) => {
                let project = self.client.project_view(task.project_id).await.unwrap();

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
        };
    }
}

pub struct Projects<'a> {
    client: &'a api::Client,
}

impl Projects<'_> {
    pub async fn list(&self) {
        match self.client.project_list().await {
            Ok(projects) => {
                for project in projects.iter() {
                    println!("{} | {}", project.id, project.name)
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        };
    }
    pub async fn view(&self, id: &String) {
        match self.client.project_view(id.to_string()).await {
            Ok(project) => {
                println!("{} | {}", project.id, project.name);
            }
            Err(e) => println!("{}", e),
        };
    }
}
