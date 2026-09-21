use std::path::Path;

use axum::{Json, http::StatusCode};

use crate::models::models::Task;

async fn root() -> &'static str {
    "<h1>Hello, World!</h1>"
}

async fn health_check() -> &'static str {
    "Ok"
}

// async fn list_tasks(Query(params): Query<HashMap<String, String>>) -> String {
//     let filter  = params.get("done").map_or("everything".to_string(), |v| format!("done={}",v));

//      format!("you requested a list of tasks with filter: {}", filter)
// }

async fn list_tasks()->Json<Vec<Task>>{
    Json(seed_task())
}

async fn get_task(Path(id): Path<u32>) -> Result<Json<Task>, StatusCode> {
    let tasks = seed_task();
    tasks
    .into_iter()
    .find(|t| t.id == id)
    .map(Json)
    .ok_or(StatusCode::NOT_FOUND)
    //Json(task.unwrap_or_else(|| Task { id, title: String::from("Task not found"), done: false }))
}

async fn create_task(Json(payload): Json<CreateTask>) -> (StatusCode, Json<Task>) {
    let new_task = Task {
        id:10,
        title:payload.title,
        done:payload.done
    };
    (StatusCode::CREATED, Json(new_task))
}

async fn update_task() -> &'static str {
    "Update task"
}
