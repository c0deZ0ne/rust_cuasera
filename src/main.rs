// struct Task {
//     id: u32,
//     title: String,
//     done:bool
// }

// fn main() {
//     let mut my_task = Task{
//         id:1,
//         title:String::from("Learn Rust"),
//         done:false 
//     };

//     print_title(&my_task);
//     let maybe_title:Option<String> = None;//Some("Buy Milk".to_string());
//     match maybe_title{
//         Some(t)=>println!("Task title: {}", t),
//         None=>println!("No title available"),
//     }

//     let parse:Result<u32,_> = "42".parse::<u32>();
//     match parse {
//         Ok (n)=>println!("Parsed number: {}", n),
//         Err(e)=>println!("Failed to parse number: {}", e)
//     }
// }


// fn print_title (task:&Task){
//     println!("Task title: {}", task.title);
// }


// async fn slow_hello()->String{
//     String::from("Hello, World!")
// }

// #[tokio::main]
// async fn main(){
//     let my_greetings = slow_hello().await;

//     println!("Greeting: {}", my_greetings);
// }

mod models;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    routing::get,
    Router,
    Json
};
use core::task;
use std::{collections::HashMap};
use serde::{Serialize,Deserialize};
#[derive(Serialize, Clone)]

struct Task {
    id: u32,
    title: String,
    done: bool
}


#[derive(Deserialize)]

//coming from the request body
struct CreateTask {
    title: String,
    done: bool
}

fn seed_task ()->Vec<Task>{
    vec![
        Task{id:1, title:String::from("Learn Rust"), done:false},
        Task{id:2, title:String::from("Learn Axum"), done:false},
        Task{id:3, title:String::from("Build a REST API"), done:false}
    ]
}

#[tokio::main]
async fn main (){
    let app_router:Router  = Router::new()
    .route("/", get(root))
    .route("/health",get(health_check))
    .route(
        "/task",
        get(list_tasks)
        .post(create_task)
        .put(update_task))
        
    .route("/task/{id}", get (get_task));

    let app_listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://{}", app_listener.local_addr().unwrap());
    axum::serve(app_listener, app_router).await.unwrap();
}

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
