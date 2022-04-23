use serde::Deserialize;

pub struct Client {
    pub http_client: reqwest::Client,
}

impl Client {
    pub async fn find(self) -> Result<Vec<Task>, reqwest::Error> {
        let resp = self
            .http_client
            .get("https://api.todoist.com/rest/v1/tasks")
            .header(
                "Authorization",
                "Bearer <TOKEN>",
            )
            .send()
            .await?
            .json::<Vec<Task>>()
            .await?;

        return Ok(resp);
    }
}

#[derive(Deserialize)]
pub struct Task {
    pub id: i64,
    pub content: String,
}
