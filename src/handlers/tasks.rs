// use std::path::Path;

// use axum::{Json, http::StatusCode};

// use crate::models::models::Task;

// async fn root() -> &'static str {
//     "<h1>Hello, World!</h1>"
// }

// async fn health_check() -> &'static str {
//     "Ok"
// }

// // async fn list_tasks(Query(params): Query<HashMap<String, String>>) -> String {
// //     let filter  = params.get("done").map_or("everything".to_string(), |v| format!("done={}",v));

// //      format!("you requested a list of tasks with filter: {}", filter)
// // }

// async fn list_tasks()->Json<Vec<Task>>{
//     Json(seed_task())
// }

// async fn get_task(Path(id): Path<u32>) -> Result<Json<Task>, StatusCode> {
//     let tasks = seed_task();
//     tasks
//     .into_iter()
//     .find(|t| t.id == id)
//     .map(Json)
//     .ok_or(StatusCode::NOT_FOUND)
//     //Json(task.unwrap_or_else(|| Task { id, title: String::from("Task not found"), done: false }))
// }

// async fn create_task(Json(payload): Json<CreateTask>) -> (StatusCode, Json<Task>) {
//     let new_task = Task {
//         id:10,
//         title:payload.title,
//         done:payload.done
//     };
//     (StatusCode::CREATED, Json(new_task))
// }

// async fn update_task() -> &'static str {
//     "Update task"
// }


use std::sync::MutexGuard;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::models::models::{Task, CreateTask, UpdateTask};
use crate::states::AppState;


pub async fn create_task (
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>
)->(StatusCode, Json<Task>) {
  let mut tasks : MutexGuard<Vec<Task>> = state.tasks.lock().unwrap(); 
  let mut next_id:MutexGuard<'_, u32> = state.next_id.lock().unwrap();
  
  let task :Task = Task {
    id:*next_id,
    title:payload.title,
    completed:false,
    description:payload.description
  };

  *next_id+=1;
  (&mut *tasks).push(task.clone());
  return (StatusCode::CREATED,Json(task));
}

pub async fn get_task_by_id (
    State(state):State<AppState>,
    Path(id):Path<u32>
)->Result<Json<Task>,StatusCode>{
    let tasks = state.tasks.lock().unwrap();
    tasks.iter()
    .find(|t|t.id==id)
    .map(|t|Json(t.clone())).ok_or(StatusCode::NOT_FOUND)
}


//update a task

pub async fn update_task(
    State(state):State<AppState>,
    Path(id):Path<u32>,
    Json(payload):Json<UpdateTask>

){
let mut tasks = state.tasks.lock().unwrap();
let task_to_edit = tasks
.iter_mut()
.find(|t| t.id==id)
.ok_or(StatusCode::NOT_FOUND)?;

if let Some(title) = payload.title {
    task_to_edit.title = title
}

if let Some (completed) = payload.completed {
    task_to_edit.completed = completed
}

if let Some (description) = payload.description {
    task_to_edit.description = description;
}

}