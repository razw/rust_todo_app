use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::{State, Path},
    Json,
    http::StatusCode,
};
use serde::Deserialize;
mod models;
mod store;

use store::TodoStore;
use models::Todo;

#[derive(Deserialize)]
struct CreateTodoRequest {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTodoRequest {
    title: Option<String>,
    completed: Option<bool>,
}

async fn handler() -> &'static str {
    "Hello, World!"
}

async fn get_todos(State(store): State<TodoStore>) -> Json<Vec<Todo>> {
    let todos = store.get_all();
    Json(todos)
}

async fn get_todo_by_id(
    State(store): State<TodoStore>,
    Path(id): Path<u32>,
) -> Json<Todo> {
    let todo = store.get_by_id(id).unwrap();
    Json(todo)
}

async fn create_todo(
    State(store): State<TodoStore>,
    Json(payload): Json<CreateTodoRequest>
) -> Json<Todo> {
    let todo = store.create(payload.title);
    Json(todo)
}

async fn update_todo(
    State(store): State<TodoStore>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTodoRequest>
) -> Result<Json<Todo>, StatusCode> {
    match store.update(id, payload.title, payload.completed) {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_todo (
    State(store): State<TodoStore>,
    Path(id): Path<u32>
) -> Result<StatusCode, StatusCode> {
    if store.delete(id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    let store = TodoStore::new();

    let app = Router::new()
        .route("/", get(handler))
        .route("/todos", get(get_todos))
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo_by_id))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo))
        .with_state(store);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
