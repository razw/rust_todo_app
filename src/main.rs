use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::{State, Path},
    Json,
    http::StatusCode,
};
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use validator::Validate;
use std::str::FromStr;
use serde::Deserialize;
mod models;
mod store;

use store::TodoStore;
use models::Todo;

#[derive(Deserialize, Validate)]
struct CreateTodoRequest {
    #[validate(length(min = 1, max = 200, message = "タイトルは1文字以上200文字以下である必要があります"))]
    title: String,
}

#[derive(Deserialize, Validate)]
struct UpdateTodoRequest {
    #[validate(length(min = 1, max = 200, message = "タイトルは1文字以上200文字以下である必要があります"))]
    title: Option<String>,
    completed: Option<bool>,
}

async fn handler() -> &'static str {
    "Hello, World!"
}

async fn get_todos(State(store): State<TodoStore>) -> Json<Vec<Todo>> {
    let todos = store.get_all().await.unwrap();
    Json(todos)
}

async fn get_todo_by_id(
    State(store): State<TodoStore>,
    Path(id): Path<u32>,
) -> Result<Json<Todo>, StatusCode> {
    match store.get_by_id(id).await {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_todo(
    State(store): State<TodoStore>,
    Json(payload): Json<CreateTodoRequest>
) -> Result<Json<Todo>, (StatusCode, Json<serde_json::Value>)> {
    if let Err(errors) = payload.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errors)| {
                errors.iter().map(|e| {
                    e.message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "Invalid value".to_string())
                })
            })
            .collect();

        return Err((
            StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Validation failed",
            "details": error_messages
            })),
        ));
    }
    match store.create(payload.title).await {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Failed to create todo"
            })),
        )),
    }
}

async fn update_todo(
    State(store): State<TodoStore>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTodoRequest>
) -> Result<Json<Todo>, (StatusCode, Json<serde_json::Value>)> {
    if let Err(errors) = payload.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errors)| {
                errors.iter().map(|e| {
                    e.message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "Invalid value".to_string())
                })
            })
            .collect();

        return Err((
            StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": "Validation failed",
            "details": error_messages
            })),
        ));
    }
    match store.update(id, payload.title, payload.completed).await {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Todo not found"
            })),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Failed to update todo"
            })),
        )),
    }
}

async fn delete_todo (
    State(store): State<TodoStore>,
    Path(id): Path<u32>
) -> Result<StatusCode, StatusCode> {
    match store.delete(id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    // ファイルベースのデータベースで永続化
    let connect_options = SqliteConnectOptions::from_str("sqlite:todos.db")
        .expect("Invalid database URL")
        .create_if_missing(true);
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    let store = TodoStore::new(pool);

    let app = Router::new()
        .route("/", get(handler))
        .route("/todos", get(get_todos))
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo_by_id))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo))
        .with_state(store);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
