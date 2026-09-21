
use crate::states::AppState;
mod routes;
mod handlers;
mod states;
mod models;

#[tokio::main]
async fn main (){
   let state = AppState::new();
   let app =routes::app(state);
    let app_listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://{}", app_listener.local_addr().unwrap());
    axum::serve(app_listener, app).await.unwrap();
}
