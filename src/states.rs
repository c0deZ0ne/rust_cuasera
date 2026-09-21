use crate::models::models::Task;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<Mutex<Vec<Task>>>,
    pub next_id: Arc<Mutex<u32>>,
}

impl AppState {
/// Creates a new instance of AppState with seeded tasks and next_id initialized to 4.

    pub fn new() -> Self {
        let seed = vec![
            Task { id: 1, title: String::from("Learn Rust"), completed: false,description: Some(String::from("Learn the basics of Rust programming language.")) },
            Task { id: 2, title: String::from("Learn Axum"), completed: false,description: Some(String::from("Learn the basics of Axum web framework.")) },
            Task { id: 3, title: String::from("Build a REST API"), completed: false,description: Some(String::from("Build a simple REST API with Rust and Axum.")) },
        ];
        //why we use self instead of AppState::new() is because we are inside the impl block of AppState,
        // so we can use Self to refer to the type we are implementing. It is a shorthand for the type name, and it makes the code more concise and easier to read.
        Self {
            tasks: Arc::new(Mutex::new(seed)),
            next_id: Arc::new(Mutex::new(4)),
        }
    }
}