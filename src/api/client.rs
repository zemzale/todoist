use crate::api::{RequestFailed, Task, TaskFilter};
use std::error::Error;
use std::ops::Add;

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

    pub async fn find(self, filter: Option<TaskFilter>) -> Result<Vec<Task>, reqwest::Error> {
        let mut path: String = "https://api.todoist.com/rest/v1/tasks".to_string();

        match filter {
            Some(f) => {
                path = path.add(&f.to_string());
            }
            None => (),
        }

        let resp = self
            .http_client
            .get(path)
            .header(self.bearer_token.0, self.bearer_token.1)
            .send()
            .await?
            .json::<Vec<Task>>()
            .await?;

        return Ok(resp);
    }

    pub async fn create(self, task: Task) -> Result<Task, reqwest::Error> {
        let path = "https://api.todoist.com/rest/v1/tasks";

        let resp = self
            .http_client
            .post(path)
            .header(self.bearer_token.0, self.bearer_token.1)
            .json(&task)
            .send()
            .await?
            .json::<Task>()
            .await?;

        return Ok(resp);
    }

    pub async fn close(self, id: i64) -> Result<(), Box<dyn Error>> {
        let path = "https://api.todoist.com/rest/v1/tasks/"
            .to_string()
            .add(&id.to_string())
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

    pub async fn view(self, id: i64) -> Result<Task, Box<dyn Error>> {
        let path: String = "https://api.todoist.com/rest/v1/tasks/"
            .to_string()
            .add(&id.to_string());

        let resp = self
            .http_client
            .get(path)
            .header(self.bearer_token.0, self.bearer_token.1)
            .send()
            .await?
            .json::<Task>()
            .await?;

        return Ok(resp);
    }
}
