use axum::{routing::get, Router};
use crate::handlers::tasks;
use crate::states::AppState;

pub fn app(state:AppState)->Router {
  
  return Router::new()
    .route("/", get(root))
    .route("/health",get(health))
    .route("/tasks",
        get(tasks::list_tasks)
        .post(tasks::create_task))

    .route("/tasks/:id",
        get(tasks::get_task)
        .put(tasks::update_task)
        .patch(tasks::update_task)
        .delete(tasks::delete_task)).with_state(state);
}


async  fn health() -> &'static str {
    "Ok"
}