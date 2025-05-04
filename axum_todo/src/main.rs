use axum::{
    routing::get,
    Router, Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener; 

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    task: String,
}

type TodoList = Arc<Mutex<Vec<Todo>>>;

async fn get_todos(State(state): State<TodoList>) -> Json<Vec<Todo>> {
    let todos = state.lock().unwrap();
    Json(todos.clone())
}

async fn create_todo(
    State(state): State<TodoList>,
    Json(payload): Json<Todo>,
) -> Json<&'static str> {
    let mut todos = state.lock().unwrap();
    todos.push(payload);
    Json("Todo created")
}

#[tokio::main]
async fn main() {
    let todo_list: TodoList = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .with_state(todo_list);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap(); 
}
