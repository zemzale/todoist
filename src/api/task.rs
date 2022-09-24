use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Add;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    #[serde(rename = "assigner_id")]
    pub assigner_id: Value,
    #[serde(rename = "assignee_id")]
    pub assignee_id: Value,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "section_id")]
    pub section_id: Value,
    #[serde(rename = "parent_id")]
    pub parent_id: Value,
    pub order: i64,
    pub content: String,
    pub description: String,
    #[serde(rename = "is_completed")]
    pub is_completed: bool,
    pub labels: Vec<String>,
    pub priority: i64,
    #[serde(rename = "comment_count")]
    pub comment_count: i64,
    #[serde(rename = "creator_id")]
    pub creator_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub due: Due,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Due {
    pub date: String,
    pub string: String,
    pub lang: String,
    #[serde(rename = "is_recurring")]
    pub is_recurring: bool,
}

pub struct TaskFilter {
    pub day_filter: String,
}

impl TaskFilter {
    pub fn to_string(self) -> String {
        let mut query: String = String::from("?filter=");
        query = query.add(&self.day_filter.to_string());
        return query;
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskCreate {
    pub content: String,
    #[serde(rename = "due_string")]
    pub due_string: Option<String>,
    pub priority: Option<i64>,
    pub project_id: Option<String>,
}

impl TaskCreate {
    pub fn new(
        content: String,
        due: String,
        priority: Option<i64>,
        project_id: Option<String>,
    ) -> TaskCreate {
        return TaskCreate {
            content,
            due_string: Some(due),
            priority,
            project_id,
        };
    }
}
impl Task {
    pub fn new(content: String) -> Task {
        return Task {
            id: String::new(),
            content,
            assigner_id: todo!(),
            assignee_id: todo!(),
            project_id: todo!(),
            section_id: todo!(),
            parent_id: todo!(),
            order: todo!(),
            description: todo!(),
            is_completed: todo!(),
            labels: todo!(),
            priority: todo!(),
            comment_count: todo!(),
            creator_id: todo!(),
            created_at: todo!(),
            due: todo!(),
            url: todo!(),
        };
    }
}
