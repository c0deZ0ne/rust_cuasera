
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)] 
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}


#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
}


#[derive(Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}