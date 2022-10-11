use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::api::{Project, RequestFailed, Task, TaskCreate, TaskFilter};
use std::error::Error;
use std::ops::Add;

use super::error::{self, ProjectNotFound};

pub struct Client {
    pub http_client: reqwest::Client,
    bearer_token: (String, String),
}

impl Client {
    pub fn new(client: reqwest::Client, token: String) -> Client {
        return Client {
            http_client: client,
            bearer_token: (
                String::from("Authorization"),
                String::from("Bearer ").add(&token),
            ),
        };
    }

    pub async fn find(&self, filter: Option<TaskFilter>) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut path: String = "/tasks".to_string();
        if let Some(f) = filter {
            path = path.add(&f.to_string());
        }

        return self.get::<Vec<Task>>(path).await;
    }

    pub async fn create(self, task: TaskCreate) -> Result<Task, Box<dyn Error>> {
        return self
            .post::<TaskCreate, Task>(Some(task), String::from("/tasks"))
            .await;
    }

    pub async fn close(self, id: String) -> Result<(), Box<dyn Error>> {
        let path = "https://api.todoist.com/rest/v2/tasks/"
            .to_string()
            .add(&id)
            .add("/close");

        let resp = self
            .http_client
            .post(path)
            .header(self.bearer_token.0, self.bearer_token.1)
            .send()
            .await;

        match resp {
            Ok(r) => {
                if !r.status().is_success() {
                    return Err(Box::new(RequestFailed::new(r.status().to_string())));
                }
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn view(&self, id: String) -> Result<Task, Box<dyn Error>> {
        let path: String = "/tasks/".to_string().add(&id);
        return self.get::<Task>(path).await;
    }

    pub async fn project_view(&self, id: String) -> Result<Project, Box<dyn Error>> {
        let path: String = "/projects/".to_string().add(&id);
        return self.get::<Project>(path).await;
    }

    pub async fn project_list(&self) -> Result<Vec<Project>, Box<dyn Error>> {
        let path: String = "/projects".to_string();
        return self.get::<Vec<Project>>(path).await;
    }

    pub async fn project_find_by_name(&self, name: String) -> Result<Project, Box<dyn Error>> {
        let path: String = "/projects".to_string();
        let projects = self.get::<Vec<Project>>(path).await?;

        for project in projects {
            if project.name == name {
                return Ok(project);
            }
        }

        Err(Box::new(ProjectNotFound::new()))
    }

    async fn get<T: DeserializeOwned>(&self, sub_path: String) -> Result<T, Box<dyn Error>> {
        let path: String = "https://api.todoist.com/rest/v2".to_string().add(&sub_path);

        let resp = self
            .http_client
            .get(path)
            .header(
                self.bearer_token.0.to_owned(),
                self.bearer_token.1.to_owned(),
            )
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(resp)
    }

    async fn post<B, T>(&self, body: Option<B>, sub_path: String) -> Result<T, Box<dyn Error>>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        let path: String = "https://api.todoist.com/rest/v2".to_string().add(&sub_path);

        let mut request = self.http_client.post(path).header(
            self.bearer_token.0.to_owned(),
            self.bearer_token.1.to_owned(),
        );

        if body.is_some() {
            request = request.json(&body)
        }

        let resp = request.send().await?.json::<T>().await?;
        Ok(resp)
    }
}
