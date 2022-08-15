use std::ops::Add;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub id: Option<i64>,
    pub content: String,
    pub due_string: Option<String>,
}

impl Task {
    pub fn new(content: String, due_string: String) -> Task {
        return Task {
            id: None,
            content,
            due_string: Some(due_string),
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
