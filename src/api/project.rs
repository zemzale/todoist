use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    #[serde(rename = "comment_count")]
    pub comment_count: i64,
    pub order: i64,
    pub color: String,
    #[serde(rename = "is_shared")]
    pub is_shared: bool,
    #[serde(rename = "is_favorite")]
    pub is_favorite: bool,
    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,
    #[serde(rename = "is_inbox_project")]
    pub is_inbox_project: bool,
    #[serde(rename = "is_team_inbox")]
    pub is_team_inbox: bool,
    #[serde(rename = "view_style")]
    pub view_style: String,
    pub url: String,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
