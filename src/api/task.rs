use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Add;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: Option<i64>,
    pub assigner: Option<i64>,
    #[serde(rename = "project_id")]
    pub project_id: Option<i64>,
    #[serde(rename = "section_id")]
    pub section_id: Option<i64>,
    pub order: Option<i64>,
    pub content: String,
    pub description: Option<String>,
    pub completed: bool,
    #[serde(rename = "label_ids")]
    pub label_ids: Vec<Value>,
    pub priority: i64,
    #[serde(rename = "comment_count")]
    pub comment_count: Option<i64>,
    pub creator: Option<i64>,
    pub created: Option<String>,
    pub due: Option<Due>,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Due {
    pub recurring: bool,
    pub string: String,
    pub date: String,
    pub datetime: Option<String>,
    pub timezone: Option<String>,
}

impl Task {
    pub fn new(content: String) -> Task {
        return Task {
            id: None,
            content,
            assigner: None,
            project_id: None,
            section_id: None,
            order: None,
            description: None,
            completed: false,
            label_ids: Vec::new(),
            priority: 0,
            comment_count: None,
            creator: None,
            created: None,
            due: None,
            url: None,
        };
    }
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
}

impl TaskCreate {
    pub fn new(content: String, due: String, priority: Option<i64>) -> TaskCreate {
        return TaskCreate {
            content,
            due_string: Some(due),
            priority,
        };
    }
}
